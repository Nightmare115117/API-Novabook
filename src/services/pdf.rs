//! Servicio de Generación de PDFs para Devoluciones a Proveedor
//!
//! Incluye metadatos completos del proveedor (RFC, teléfono, correo, dirección, contacto),
//! desglose de artículos devueltos y firmas de autorización.

use crate::productos::model::{Devolucion, TipoProducto};

pub struct PdfService;

impl PdfService {
    /// Genera los bytes de un PDF estándar correspondiente a la devolución a proveedor
    pub fn generar_pdf_devolucion(devolucion: &Devolucion, tipo_producto: TipoProducto) -> Vec<u8> {
        let tipo_str = tipo_producto.as_str().to_uppercase();
        let titulo = format!("NOVABOOK - ACTA DE DEVOLUCION A PROVEEDOR ({})", tipo_str);
        let subtitulo = "Libreria y Papeleria NovaBook - Sucursal Central";

        let prov_nombre = devolucion
            .nombre_proveedor
            .as_deref()
            .unwrap_or("Proveedor General");

        let prov_rfc = devolucion.proveedor_rfc.as_deref().unwrap_or("N/D");

        let prov_tel = devolucion.proveedor_telefono.as_deref().unwrap_or("N/D");

        let prov_correo = devolucion.proveedor_correo.as_deref().unwrap_or("N/D");

        let prov_dir = devolucion.proveedor_direccion.as_deref().unwrap_or("N/D");

        let prov_contacto = devolucion
            .proveedor_contacto
            .as_deref()
            .unwrap_or("Atencion a Proveedores");

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
        lines.push(
            "================================================================================"
                .to_string(),
        );
        lines.push(format!("ID Devolucion:   #{}", devolucion.id_devolucion));
        lines.push(format!("Fecha:           {}", devolucion.fecha));
        lines.push(format!("Vendedor:        {}", vend_nombre));
        lines.push(format!("Estado:          {}", devolucion.estado));
        lines.push(format!("Autorizado por:  {}", autorizador));
        lines.push(
            "--------------------------------------------------------------------------------"
                .to_string(),
        );
        lines.push("DATOS INSTITUCIONALES DEL PROVEEDOR:".to_string());
        lines.push(format!("Razon Social:    {}", prov_nombre));
        lines.push(format!("RFC / Id Fiscal: {}", prov_rfc));
        lines.push(format!("Telefono:        {}", prov_tel));
        lines.push(format!("Correo:          {}", prov_correo));
        lines.push(format!("Direccion:       {}", prov_dir));
        lines.push(format!("Contacto:        {}", prov_contacto));
        lines.push(
            "--------------------------------------------------------------------------------"
                .to_string(),
        );
        lines.push("DETALLE DE ARTICULOS:".to_string());
        lines.push(format!(
            "{:<16} {:<10} {:<36} {:>6}",
            "CODIGO EAN", "SKU", "TITULO", "CANT"
        ));
        lines.push(
            "--------------------------------------------------------------------------------"
                .to_string(),
        );

        if devolucion.items.is_empty() {
            lines.push(format!(
                "(Total piezas declaradas en lote: {})",
                devolucion.total_piezas
            ));
        } else {
            for item in &devolucion.items {
                let sku_str = item
                    .sku
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "N/A".to_string());
                let titulo_trunc = if item.titulo.len() > 34 {
                    format!("{}...", &item.titulo[..31])
                } else {
                    item.titulo.clone()
                };
                lines.push(format!(
                    "{:<16} {:<10} {:<36} {:>6}",
                    item.codigo_ean, sku_str, titulo_trunc, item.cantidad
                ));
            }
        }

        lines.push(format!(
            "TOTAL DE PIEZAS A DEVOLVER: {}",
            devolucion.total_piezas
        ));
        lines.push(
            "================================================================================"
                .to_string(),
        );
        lines.push("".to_string());
        lines.push("FIRMAS DE CONFORMIDAD Y ENTREGA:".to_string());
        lines.push("".to_string());
        lines.push(
            "_____________________________                _____________________________"
                .to_string(),
        );
        lines.push(
            "   Firma Vendedor Solicitante                   Firma Jefe de Departamento"
                .to_string(),
        );

        Self::construir_pdf_binario(&lines)
    }

    /// Construye una estructura binaria PDF-1.4 válida con objetos de texto
    fn construir_pdf_binario(lineas: &[String]) -> Vec<u8> {
        let mut stream_content = String::new();
        stream_content.push_str("BT\n/F1 10 Tf\n50 740 Td\n14 TL\n");

        for linea in lineas {
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

        let mut pdf = Vec::new();
        pdf.extend_from_slice(b"%PDF-1.4\n");

        let mut offsets = Vec::new();

        offsets.push(pdf.len());
        pdf.extend_from_slice(obj1.as_bytes());

        offsets.push(pdf.len());
        pdf.extend_from_slice(obj2.as_bytes());

        offsets.push(pdf.len());
        pdf.extend_from_slice(obj3.as_bytes());

        offsets.push(pdf.len());
        pdf.extend_from_slice(obj4_header.as_bytes());
        pdf.extend_from_slice(stream_bytes);
        pdf.extend_from_slice(obj4_footer.as_bytes());

        offsets.push(pdf.len());
        pdf.extend_from_slice(obj5.as_bytes());

        let xref_offset = pdf.len();
        pdf.extend_from_slice(b"xref\n0 6\n");
        pdf.extend_from_slice(b"0000000000 65535 f \n");

        for offset in offsets {
            let entry = format!("{:010} 00000 n \n", offset);
            pdf.extend_from_slice(entry.as_bytes());
        }

        pdf.extend_from_slice(b"trailer\n<< /Size 6 /Root 1 0 R >>\nstartxref\n");
        let startxref_str = format!("{}\n%%EOF\n", xref_offset);
        pdf.extend_from_slice(startxref_str.as_bytes());

        pdf
    }
}
