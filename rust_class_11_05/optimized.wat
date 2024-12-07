(module
 (type $0 (func (param i32 i32) (result i32)))
 (global $global$1 i32 (i32.const 65536))
 (global $global$2 i32 (i32.const 65536))
 (export "add" (func $0))
 (export "__data_end" (global $global$1))
 (export "__heap_base" (global $global$2))
 (func $0 (param $0 i32) (param $1 i32) (result i32)
  (i32.add
   (local.get $0)
   (local.get $1)
  )
 )
)
