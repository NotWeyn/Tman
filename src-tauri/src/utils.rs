use qrcode::QrCode;
use image::Luma;
use std::net::UdpSocket;
use base64::{Engine as _, engine::general_purpose};

pub fn get_local_ip() -> Option<String> {
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(_) => return None,
    };

    match socket.connect("8.8.8.8:80") {
        Ok(()) => (),
        Err(_) => return None,
    };

    match socket.local_addr() {
        Ok(addr) => Some(addr.ip().to_string()),
        Err(_) => None,
    }
}

pub fn generate_qr_base64(data: &str) -> Result<String, String> {
    let code = QrCode::new(data).map_err(|e| e.to_string())?;
    let image = code.render::<Luma<u8>>().build();
    
    let mut bytes: Vec<u8> = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut bytes);
    
    image::DynamicImage::ImageLuma8(image)
        .write_to(&mut cursor, image::ImageFormat::Png)
        .map_err(|e| e.to_string())?;
        
    let b64 = general_purpose::STANDARD.encode(&bytes);
    Ok(format!("data:image/png;base64,{}", b64))
}
pub fn dynamic_image_to_base64(image: &image::DynamicImage) -> Result<String, String> {
    let mut bytes: Vec<u8> = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut bytes);
    
    image.write_to(&mut cursor, image::ImageFormat::Png)
        .map_err(|e| e.to_string())?;
        
    let b64 = general_purpose::STANDARD.encode(&bytes);
    Ok(format!("data:image/png;base64,{}", b64))
}
