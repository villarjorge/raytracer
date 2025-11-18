from pathlib import Path

if __name__ == "__main__":
    p = Path("src")
    for file_path in p.glob("**/*.rs"):
        with open(file_path, "r") as file:
            for line_number, line in enumerate(file.read().split("\n"), 1):
                line = line.strip()
                if line.startswith("// To do"): print(f"In {file_path}:{line_number} {line}")