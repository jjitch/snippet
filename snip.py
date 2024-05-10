import os
import json
from typing import Dict
import glob

app_data = os.environ["APPDATA"]

snip_file: str = f"{app_data}\\Code\\User\\snippets\\rust.json"
library_pattern = "indexer"


def test(file_no_ext: str) -> bool:
    if file_no_ext == "main" or file_no_ext == "lib":
        return False
    cmd = f"cargo +1.70.0 test {file_no_ext}_test::"
    print(cmd)
    retval = os.system(cmd)
    return retval == 0


def contents(rust_file: str) -> str:
    f = open(rust_file)
    raw = ["// " + x if x.startswith("use super::") else x for x in f.readlines()]
    contents = "".join(raw).replace("$", "\\$").replace("    ", "\t")
    return contents


def main():
    crate_path: str = os.getcwd()
    src_path = crate_path + "\\src\\"
    fr = open(snip_file, "r", encoding="utf-8")
    obj: Dict = json.load(fr)
    fr.close()

    for rust_file in glob.glob(src_path + library_pattern + ".rs"):
        basename = os.path.basename(rust_file)
        file_no_ext = os.path.splitext(basename)[0]
        if test(file_no_ext):
            snip_obj = {}
            snip_obj["prefix"] = f"lib_{file_no_ext}"
            snip_obj["body"] = contents(rust_file)
            obj[f"lib_{file_no_ext}"] = snip_obj

    fw = open(snip_file, "w", encoding="utf-8")
    json.dump(obj, fw, indent=4)


if __name__ == "__main__":
    main()
