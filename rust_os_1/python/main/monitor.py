
import os
import sys
import subprocess
import time 

REG_FILE = "registry.txt"
INTERVAL = 3

def read_registry():
    if not os.path.exists(REG_FILE):
        return None, None
    with open(REG_FILE,"r") as f:
        line = f.readline().strip()
    if not line:
        return None, None

    parts = line.split()
    pid = int(parts[0])
    cmd = parts[1:]
    return pid , cmd

def process_alive(pid):
    try:
        os.kill(pid, 0)
        return True
    except ProcessLookupError:
        return False
    except PermissionError:
        return True

def restart_process(cmd):
    print(f"Restartingt Process: '{' '.join(cmd)}'")
    process = subprocess.Popen(cmd)
    with open(REG_FILE, "w") as f:
        f.write(f"{process.pid} {' '.join(cmd)}\n")
    return process.pid

def main():
    print("Monitoring Started .....")

    while True:
        pid, cmd = read_registry()

        if pid is None:
            time.sleep(INTERVAL)
            continue
        if not process_alive(pid):
            print(f"Process {pid} is dead.")
            restart_process(cmd)

        time.sleep(INTERVAL)

if __name__ == "__main__":
    main()
