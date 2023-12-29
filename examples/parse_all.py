from android_bp import BluePrint
import os, sys
import time
t1 = time.time()
num_files = 0
for root, dirs, files in os.walk(sys.argv[1] if len(sys.argv) > 1 else "."):
    for file in files:
        if file == "Android.bp":
            b = BluePrint.from_file(os.path.join(root, file))
            print(b.variables)
            print([bl.__type__ for bl in b.modules])
            for bl in b.modules_by_type("rust_binary"):
                print(bl.name, bl.srcs)
            num_files += 1
print("time: ", time.time() - t1, "num_files: ", num_files)
