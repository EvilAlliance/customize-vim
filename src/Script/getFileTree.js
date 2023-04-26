import { readDir, exists } from "@tauri-apps/api/fs"
import { invoke } from "@tauri-apps/api/tauri"
import { extname, resolveResource } from '@tauri-apps/api/path';
import { Extension } from "./Constantes/Extention.js";

export async function getFileTree() {
    const Disks = await getDisks();
    let FileTree = new Array();
    Disks.forEach(async Disk => {
        FileTree = FileTree.concat(Array.from(await FormFileTree(Disk)));
    });
}

async function getDisks() {
    const Disks = new Set();
    const AsciiCodeA = 65;
    const AsciiCodeZ = 90;
    for (let i = AsciiCodeA; i <= AsciiCodeZ; i++) {
        const route = String.fromCharCode(i) + ':/';
        if (await exists(route)) {
            Disks.add(route);
        }
    }
    return Disks;
}

async function FormFileTree(File) {
    const List = new Set();
    const FileTree = JSON.parse(await invoke("open_folder", { folderPath: File }));
    console.log(FileTree)
    const GetPaths = (el) => {
        if (el.kind === 'Directory') {
            List.add(el.path);
            el.children.forEach(GetPaths);
        } else {
            const ext = el.path.split('.');
            if (Extension.includes(ext[ext.length - 1])) {
                List.add(el.path)
            }
        }
    }

    await FileTree.forEach(GetPaths);
    return List;
}
