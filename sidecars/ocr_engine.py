import sys
import os

try:
    from rapidocr_onnxruntime import RapidOCR
except ImportError:
    print("Error: rapidocr_onnxruntime is not installed. Run 'uv pip install rapidocr_onnxruntime'", file=sys.stderr)
    sys.exit(1)

def main():
    if len(sys.argv) < 2:
        print("Usage: python ocr_engine.py <image_path>", file=sys.stderr)
        sys.exit(1)

    image_path = sys.argv[1]
    
    if not os.path.exists(image_path):
        print(f"Error: Image not found at {image_path}", file=sys.stderr)
        sys.exit(1)

    engine = RapidOCR()
    result, elapse = engine(image_path)
    
    if not result:
        print("")
        return

    # result is typically a tuple: (dt_boxes, rec_res, scores)
    # the second element of each item is the text
    extracted_text = []
    for item in result:
        text = item[1]
        extracted_text.append(text)
        
    print(" ".join(extracted_text))

if __name__ == "__main__":
    main()
