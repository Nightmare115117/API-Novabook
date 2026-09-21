//! Servicio de Generación de PDFs para Devoluciones a Proveedor
//!
//! ### Crate recomendada para generación de PDF en Rust:
//! En un entorno de producción en Rust, las crates líderes recomendadas son:
//! 1. `printpdf`: Excelente para control tipográfico, inclusión de fuentes TrueType/Type1, gráficos y salida PDF/A estricta.
//! 2. `genpdf`: Crate de nivel superior inspirada en ReportLab de Python, que maneja flujo de texto automático, saltos de página y tablas dinámicas.
//!
//! A continuación se implementa la generación de un documento PDF estructurado
//! en formato binario estándar PDF-1.4 con cabeceras institucionales, datos de la devolución,
//! tabla de artículos (Libros o Revistas) y líneas de firma de autorización.

use crate::productos::model::{Devolucion, TipoProducto};

pub struct PdfService;

impl PdfService {
    /// Genera los bytes de un PDF estándar correspondiente a la devolución a proveedor
    pub fn generar_pdf_devolucion(
        devolucion: &Devolucion,
        tipo_producto: TipoProducto,
    ) -> Vec<u8> {
        let tipo_str = tipo_producto.as_str().to_uppercase();
        let titulo = format!("NOVABOOK - REPORTE DE DEVOLUCION A PROVEEDOR ({})", tipo_str);
        let subtitulo = "Libreria y Papeleria - Sanborns Saltillo";

        let prov_nombre = devolucion
            .nombre_proveedor
            .as_deref()
            .unwrap_or("Proveedor General");

        let vend_nombre = devolucion
            .vendedor_nombre
            .as_deref()
            .unwrap_or("Vendedor Asignado");

        let autorizador = devolucion
            .autorizado_por
            .as_deref()
            .unwrap_or("Pendiente de Autorizacion");

        // Construir contenido textual formateado del PDF
        let mut lines = Vec::new();
        lines.push(titulo);
        lines.push(subtitulo.to_string());
        lines.push("================================================================================".to_string());
        lines.push(format!("ID Devolucion:   #{}", devolucion.id_devolucion));
        lines.push(format!("Fecha:           {}", devolucion.fecha));
        lines.push(format!("Proveedor:       {}", prov_nombre));
        lines.push(format!("Vendedor:        {}", vend_nombre));
        lines.push(format!("Total Piezas:    {}", devolucion.total_piezas));
        lines.push(format!("Estado:          {}", devolucion.estado));
        lines.push(format!("Autorizado por:  {}", autorizador));
        lines.push("--------------------------------------------------------------------------------".to_string());
        lines.push("DETALLE DE ARTICULOS:".to_string());
        lines.push(format!("{:<16} {:<10} {:<36} {:>6}", "CODIGO EAN", "SKU", "TITULO", "CANT"));
        lines.push("--------------------------------------------------------------------------------".to_string());

        if devolucion.items.is_empty() {
            lines.push(format!("(Sin desglose individual registrado - Total piezas declaradas: {})", devolucion.total_piezas));
        } else {
            for item in &devolucion.items {
                let sku_str = item.sku.map(|s| s.to_string()).unwrap_or_else(|| "N/A".to_string());
                let titulo_trunc = if item.titulo.len() > 34 {
                    format!("{}...", &item.titulo[..31])
                } else {
                    item.titulo.clone()
                };
                lines.push(format!("{:<16} {:<10} {:<36} {:>6}", item.codigo_ean, sku_str, titulo_trunc, item.cantidad));
            }
        }

        lines.push("================================================================================".to_string());
        lines.push("".to_string());
        lines.push("FIRMAS DE CONFORMIDAD:".to_string());
        lines.push("".to_string());
        lines.push("_____________________________                _____________________________".to_string());
        lines.push("   Firma Vendedor Solicitante                   Firma Jefe de Departamento".to_string());

        Self::construir_pdf_binario(&lines)
    }

    /// Construye una estructura binaria PDF-1.4 válida con objetos de texto
    fn construir_pdf_binario(lineas: &[String]) -> Vec<u8> {
        let mut stream_content = String::new();
        stream_content.push_str("BT\n/F1 10 Tf\n50 740 Td\n15 TL\n");

        for linea in lineas {
            // Escapar paréntesis y barras diagonales inversas para la sintaxis PDF
            let escaped = linea
                .replace('\\', "\\\\")
                .replace('(', "\\(")
                .replace(')', "\\)");
            stream_content.push_str(&format!("({}) ' \n", escaped));
        }

        stream_content.push_str("ET\n");

        let stream_bytes = stream_content.as_bytes();
        let stream_len = stream_bytes.len();

        let obj1 = "1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n";
        let obj2 = "2 0 obj\n<< /Type /Pages /Kids [3 0 R] /Count 1 >>\nendobj\n";
        let obj3 = "3 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Contents 4 0 R /Resources << /Font << /F1 5 0 R >> >> >>\nendobj\n";
        let obj4_header = format!("4 0 obj\n<< /Length {} >>\nstream\n", stream_len);
        let obj4_footer = "\nendstream\nendobj\n";
        let obj5 = "5 0 obj\n<< /Type /Font /Subtype /Type1 /BaseFont /Courier >>\nendobj\n";

        let mut body = Vec::new();
        let header = b"%PDF-1.4\n%\xE2\xE3\xCF\xD3\n";
        body.extend_from_slice(header);

        let mut offsets = Vec::new();

        // Obj 1
        offsets.push(body.len());
        body.extend_from_slice(obj1.as_bytes());

        // Obj 2
        offsets.push(body.len());
        body.extend_from_slice(obj2.as_bytes());

        // Obj 3
        offsets.push(body.len());
        body.extend_from_slice(obj3.as_bytes());

        // Obj 4
        offsets.push(body.len());
        body.extend_from_slice(obj4_header.as_bytes());
        body.extend_from_slice(stream_bytes);
        body.extend_from_slice(obj4_footer.as_bytes());

        // Obj 5
        offsets.push(body.len());
        body.extend_from_slice(obj5.as_bytes());

        let xref_offset = body.len();
        let mut xref = String::new();
        xref.push_str("xref\n0 6\n");
        xref.push_str("0000000000 65535 f \n");
        for offset in offsets {
            xref.push_str(&format!("{:010} 00000 n \n", offset));
        }

        let trailer = format!(
            "trailer\n<< /Size 6 /Root 1 0 R >>\nstartxref\n{}\n%%EOF\n",
            xref_offset
        );

        body.extend_from_slice(xref.as_bytes());
        body.extend_from_slice(trailer.as_bytes());

        body
    }
}
