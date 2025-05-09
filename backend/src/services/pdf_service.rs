use printpdf::*;
use std::io::BufWriter;

use crate::{
    errors::AppError,
    models::Receipt,
};

#[derive(Clone)]
pub struct PdfService;

impl PdfService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn generate_receipt_pdf(&self, receipt: &Receipt) -> Result<Vec<u8>, AppError> {
        let (doc, page1, layer1) = PdfDocument::new(
            format!("Receipt_{}", receipt.id),
            Mm(210.0),
            Mm(297.0),
            "Layer 1",
        );
        let current_layer = doc.get_page(page1).get_layer(layer1);

        let font = doc.add_builtin_font(BuiltinFont::Helvetica).map_err(|e| AppError::PdfGenerationError(format!("Failed to load built-in font: {}",e)))?;

        current_layer.set_text_cursor(Mm(20.0), Mm(270.0)); 
        current_layer.set_font(&font, 24.0);
        current_layer.write_text("Receipt Details", &font);

        current_layer.add_line_break();
        current_layer.add_line_break();
        current_layer.set_font(&font, 12.0);

        current_layer.write_text(format!("Receipt ID: {}", receipt.id), &font);
        current_layer.add_line_break();
        current_layer.write_text(format!("User ID: {}", receipt.user_id), &font);
        current_layer.add_line_break();
        current_layer.write_text(format!("Title: {}", receipt.title), &font); // Added Title
        current_layer.add_line_break();
        current_layer.write_text(format!("Date: {}", receipt.date.format("%Y-%m-%d %H:%M:%S UTC")), &font); // Formatted date
        current_layer.add_line_break();
        current_layer.write_text(format!("Amount: ${:.2}", receipt.amount), &font);
        current_layer.add_line_break();
        current_layer.write_text(format!("Description: {}", receipt.description.as_deref().unwrap_or("N/A")), &font); // Handle Option<String>
        current_layer.add_line_break();
        current_layer.write_text(format!("Category: {}", receipt.category.as_deref().unwrap_or("N/A")), &font); // Handle Option<String>
        current_layer.add_line_break();
        current_layer.write_text(format!("File URL: {}", receipt.file_url.as_deref().unwrap_or("N/A")), &font); // Handle Option<String>
        current_layer.add_line_break();
        current_layer.add_line_break();
        current_layer.write_text(format!("Generated on: {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")), &font); // Formatted date

        let mut buffer = BufWriter::new(Vec::new());
        doc.save(&mut buffer).map_err(|e| AppError::PdfGenerationError(format!("Failed to save PDF: {}",e)))?;
        
        buffer.into_inner().map_err(|e| AppError::PdfGenerationError(format!("Failed to get PDF bytes: {}", e.to_string())))
    }
}

