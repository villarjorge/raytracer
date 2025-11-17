from pathlib import Path

if __name__ == "__main__":
    p = Path("src")
    for file_path in p.glob("**/*.rs"):
        print(f"In {file_path}:")
        with open(file_path, "r") as file:
            for line_number, line in enumerate(file.read().split("\n"), 1):
                if line.startswith("// To do"): print(f"    Line: {line_number} {line}")