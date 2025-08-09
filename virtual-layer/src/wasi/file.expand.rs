mod ex {
    use crate::{
        ConstFiles, wasi::file::non_atomic::{VFSConstNormalFiles, WasiConstFile},
    };
    const FILES: VFSConstNormalFiles<WasiConstFile<&'static str>, 10> = crate::wasi::file::non_atomic::VFSConstNormalFiles::new({
        let mut static_array = crate::binary_map::StaticArrayBuilder::new();
        let empty_arr = {
            let mut empty_arr = crate::binary_map::StaticArrayBuilder::new();
            empty_arr.push("/root/root.txt");
            empty_arr.push("/root");
            empty_arr.push("./hey");
            empty_arr.push("./hello/world");
            empty_arr.push("./hello/everyone");
            empty_arr.push("./hello");
            empty_arr.push(".");
            empty_arr.push("~/home");
            empty_arr.push("~/user");
            empty_arr.push("~");
            empty_arr.build()
        };
        static_array
            .push((
                "/root/root.txt",
                crate::wasi::file::non_atomic::VFSConstNormalInode::File({
                    WasiConstFile::new("This is root")
                }),
            ));
        static_array
            .push((
                "/root",
                crate::wasi::file::non_atomic::VFSConstNormalInode::Dir(
                    crate::wasi::file::non_atomic::vfs_const_macro_fn(
                        empty_arr,
                        "/root",
                        &static_array,
                    ),
                ),
            ));
        static_array
            .push((
                "./hey",
                crate::wasi::file::non_atomic::VFSConstNormalInode::File({
                    WasiConstFile::new("Hey!")
                }),
            ));
        static_array
            .push((
                "./hello/world",
                crate::wasi::file::non_atomic::VFSConstNormalInode::File({
                    WasiConstFile::new("Hello, world!")
                }),
            ));
        static_array
            .push((
                "./hello/everyone",
                crate::wasi::file::non_atomic::VFSConstNormalInode::File({
                    WasiConstFile::new("Hello, everyone!")
                }),
            ));
        static_array
            .push((
                "./hello",
                crate::wasi::file::non_atomic::VFSConstNormalInode::Dir(
                    crate::wasi::file::non_atomic::vfs_const_macro_fn(
                        empty_arr,
                        "./hello",
                        &static_array,
                    ),
                ),
            ));
        static_array
            .push((
                ".",
                crate::wasi::file::non_atomic::VFSConstNormalInode::Dir(
                    crate::wasi::file::non_atomic::vfs_const_macro_fn(
                        empty_arr,
                        ".",
                        &static_array,
                    ),
                ),
            ));
        static_array
            .push((
                "~/home",
                crate::wasi::file::non_atomic::VFSConstNormalInode::File({
                    WasiConstFile::new("This is home")
                }),
            ));
        static_array
            .push((
                "~/user",
                crate::wasi::file::non_atomic::VFSConstNormalInode::File({
                    WasiConstFile::new("This is user")
                }),
            ));
        static_array
            .push((
                "~",
                crate::wasi::file::non_atomic::VFSConstNormalInode::Dir(
                    crate::wasi::file::non_atomic::vfs_const_macro_fn(
                        empty_arr,
                        "~",
                        &static_array,
                    ),
                ),
            ));
        static_array.build()
    });
}
