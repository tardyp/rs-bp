# equivalent for --example parse_all
import android_bp
import os, sys
import time
t1 = time.time()
num_files = 0
for root, dirs, files in os.walk(sys.argv[1] if len(sys.argv) > 1 else "."):
    for file in files:
        if file == "Android.bp":
            with open(os.path.join(root, file)) as f:
                b = android_bp.PyBluePrint(f.read())
                num_files += 1
print("time: ", time.time() - t1, "num_files: ", num_files)
