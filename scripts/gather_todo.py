"""
A script that looks for all occurences of the string "// To do:" and prints them. 
It also prints the file path and line number in a way that VScode can recognize, 
which means you can click on them to take you to the location of the to do.
In this state it is more or less the same as the search function in VScode
"""

from pathlib import Path

if __name__ == "__main__":
    p = Path("src")
    for file_path in p.glob("**/*.rs"):
        with open(file_path, "r") as file:
            for line_number, line in enumerate(file.read().split("\n"), 1):
                line = line.strip()
                if line.startswith("// To do"): print(f"In {file_path}:{line_number} {line}")