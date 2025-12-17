"""
Build and run the project with the AMDμ Prof CLI.
"""

import subprocess

if __name__ == "__main__":
    # Build the current setup, do profiling and print the result
    subprocess.run(["cargo", "build", "--release"])
    # https://docs.amd.com/r/en-US/57368-uProf-user-guide/Options?tocId=YElGBDlUd7v%7E7sLd7VCs2g
    command = r"AMDuProfCLI.exe profile --config hotspots --timer-interval 10 --stdout --working-dir C:\Users\villa\Documents\Rust\raytracer --output-dir C:\Users\villa\AppData\Roaming\AMDuProf\raytracer\ C:\Users\villa\Documents\Rust\raytracer\target\release\raytracer.exe "
    command = command.split(" ")
    # https://docs.python.org/3/library/subprocess.html#subprocess.run
    completed = subprocess.run(command, text=True)