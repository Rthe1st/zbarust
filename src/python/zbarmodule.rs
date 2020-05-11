use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type PyMemberDef;
    /* built-in 'super' */
    #[no_mangle]
    fn PyType_Ready(_: *mut PyTypeObject) -> libc::c_int;
    #[no_mangle]
    fn PyObject_IsTrue(_: *mut PyObject) -> libc::c_int;
    /*
_Py_NoneStruct is an object of undefined type which can be used in contexts
where NULL (nil) is not suitable (since NULL often means 'error').

Don't forget to apply Py_INCREF() when returning this value!!!
*/
    #[no_mangle]
    static mut _Py_NoneStruct: PyObject;
    #[no_mangle]
    static mut zbarScanner_Type: PyTypeObject;
    #[no_mangle]
    static mut zbarDecoder_Type: PyTypeObject;
    #[no_mangle]
    static mut zbarImageScanner_Type: PyTypeObject;
    #[no_mangle]
    static mut zbarProcessor_Type: PyTypeObject;
    #[no_mangle]
    static mut zbarSymbolIter_Type: PyTypeObject;
    #[no_mangle]
    fn zbarSymbol_LookupEnum(type_0: zbar_symbol_type_t) -> *mut zbarEnumItem;
    #[no_mangle]
    static mut zbarSymbol_Type: PyTypeObject;
    #[no_mangle]
    static mut zbarSymbolSet_Type: PyTypeObject;
    #[no_mangle]
    static mut zbarImage_Type: PyTypeObject;
    #[no_mangle]
    fn zbarEnum_Add(self_0: *mut zbarEnum, val: libc::c_int,
                    name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn zbarEnum_New() -> *mut zbarEnum;
    #[no_mangle]
    static mut zbarEnum_Type: PyTypeObject;
    #[no_mangle]
    fn zbarEnumItem_New(byname: *mut PyObject, byvalue: *mut PyObject,
                        val: libc::c_int, name: *const libc::c_char)
     -> *mut zbarEnumItem;
    #[no_mangle]
    static mut zbarEnumItem_Type: PyTypeObject;
    #[no_mangle]
    static mut zbarException_Type: PyTypeObject;
    #[no_mangle]
    static mut PyInt_Type: PyTypeObject;
    #[no_mangle]
    fn PyInt_AsSsize_t(_: *mut PyObject) -> Py_ssize_t;
    /* This excludes Values, since they are not sets. */
    #[no_mangle]
    fn PyDict_New() -> *mut PyObject;
    #[no_mangle]
    fn PyModule_GetDict(_: *mut PyObject) -> *mut PyObject;
    #[no_mangle]
    fn PyErr_Occurred() -> *mut PyObject;
    #[no_mangle]
    static mut PyExc_Exception: *mut PyObject;
    /* Mask the old API with a call to the new API for code compiled under
   Python 2.0: */
    /* Function to create a new exception */
    #[no_mangle]
    fn PyErr_NewException(name: *mut libc::c_char, base: *mut PyObject,
                          dict: *mut PyObject) -> *mut PyObject;
    #[no_mangle]
    fn PyArg_ParseTuple(_: *mut PyObject, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn Py_BuildValue(_: *const libc::c_char, _: ...) -> *mut PyObject;
    #[no_mangle]
    fn PyModule_AddObject(_: *mut PyObject, _: *const libc::c_char,
                          _: *mut PyObject) -> libc::c_int;
    #[no_mangle]
    fn Py_InitModule4_64(name: *const libc::c_char, methods: *mut PyMethodDef,
                         doc: *const libc::c_char, self_0: *mut PyObject,
                         apiver: libc::c_int) -> *mut PyObject;
    /*
     Returns the integer n converted to a string with a base, with a base
     marker of 0b, 0o or 0x prefixed if applicable.
     If n is not an int object, it is converted with PyNumber_Index first.
       */
    /*  Sequence protocol:*/
    #[no_mangle]
    fn PySequence_Check(o: *mut PyObject) -> libc::c_int;
    /*
     Return 1 if the object provides sequence protocol, and zero
     otherwise.

     This function always succeeds.

       */
    #[no_mangle]
    fn PySequence_Size(o: *mut PyObject) -> Py_ssize_t;
    /*
     Return the result of repeating sequence object o count times,
     or NULL on failure.  This is the equivalent of the Python
     expression: o1*count.

       */
    #[no_mangle]
    fn PySequence_GetItem(o: *mut PyObject, i: Py_ssize_t) -> *mut PyObject;
    /* * retrieve runtime library version information.
 * @param major set to the running major version (unless NULL)
 * @param minor set to the running minor version (unless NULL)
 * @returns 0
 */
    #[no_mangle]
    fn zbar_version(major: *mut libc::c_uint, minor: *mut libc::c_uint)
     -> libc::c_int;
    /* * set global library debug level.
 * @param verbosity desired debug level.  higher values create more spew
 */
    #[no_mangle]
    fn zbar_set_verbosity(verbosity: libc::c_int);
    /* * increase global library debug level.
 * eg, for -vvvv
 */
    #[no_mangle]
    fn zbar_increase_verbosity();
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type ssize_t = __ssize_t;
/* include for defines */
/* Some versions of HP-UX & Solaris need inttypes.h for int32_t,
   INT32_MAX, etc. */
/* *************************************************************************
Symbols and macros to supply platform-independent interfaces to basic
C language & library operations whose spellings vary across platforms.

Please try to make documentation here as clear as possible:  by definition,
the stuff here is trying to illuminate C's darkest corners.

Config #defines referenced here:

SIGNED_RIGHT_SHIFT_ZERO_FILLS
Meaning:  To be defined iff i>>j does not extend the sign bit when i is a
          signed integral type and i < 0.
Used in:  Py_ARITHMETIC_RIGHT_SHIFT

Py_DEBUG
Meaning:  Extra checks compiled in for debug mode.
Used in:  Py_SAFE_DOWNCAST

HAVE_UINTPTR_T
Meaning:  The C9X type uintptr_t is supported by the compiler
Used in:  Py_uintptr_t

HAVE_LONG_LONG
Meaning:  The compiler supports the C type "long long"
Used in:  PY_LONG_LONG

**************************************************************************/
/* For backward compatibility only. Obsolete, do not use. */
/* typedefs for some C9X-defined synonyms for integral types.
 *
 * The names in Python are exactly the same as the C9X names, except with a
 * Py_ prefix.  Until C9X is universally implemented, this is the only way
 * to ensure that Python gets reliable names that don't conflict with names
 * in non-Python code that are playing their own tricks to define the C9X
 * names.
 *
 * NOTE: don't go nuts here!  Python has no use for *most* of the C9X
 * integral synonyms.  Only define the ones we actually need.
 */
/* If LLONG_MAX is defined in limits.h, use that. */
/* LLONG_MAX */
/* HAVE_LONG_LONG */
/* a build with 30-bit digits for Python long integers needs an exact-width
 * 32-bit unsigned integer type to store those digits.  (We could just use
 * type 'unsigned long', but that would be wasteful on a system where longs
 * are 64-bits.)  On Unix systems, the autoconf macro AC_TYPE_UINT32_T defines
 * uint32_t to be such a type unless stdint.h or inttypes.h defines uint32_t.
 * However, it doesn't set HAVE_UINT32_T, so we do that here.
 */
/* Macros for a 64-bit unsigned integer type; used for type 'twodigits' in the
 * long integer implementation, when 30-bit digits are enabled.
 */
/* Signed variants of the above */
/* If PYLONG_BITS_IN_DIGIT is not defined then we'll use 30-bit digits if all
   the necessary integer types are available, and we're on a 64-bit platform
   (as determined by SIZEOF_VOID_P); otherwise we use 15-bit digits. */
/* uintptr_t is the C9X name for an unsigned integral type such that a
 * legitimate void* can be cast to uintptr_t and then back to void* again
 * without loss of information.  Similarly for intptr_t, wrt a signed
 * integral type.
 */
/* HAVE_UINTPTR_T */
/* Py_ssize_t is a signed integral type such that sizeof(Py_ssize_t) ==
 * sizeof(size_t).  C99 doesn't define such a thing directly (size_t is an
 * unsigned integral type).  See PEP 353 for details.
 */
pub type Py_ssize_t = ssize_t;
/* Object and type object interface */
/*
Objects are structures allocated on the heap.  Special rules apply to
the use of objects to ensure they are properly garbage-collected.
Objects are never allocated statically or on the stack; they must be
accessed through special macros and functions only.  (Type objects are
exceptions to the first rule; the standard types are represented by
statically initialized type objects, although work on type/class unification
for Python 2.2 made it possible to have heap-allocated type objects too).

An object has a 'reference count' that is increased or decreased when a
pointer to the object is copied or deleted; when the reference count
reaches zero there are no references to the object left and it can be
removed from the heap.

An object has a 'type' that determines what it represents and what kind
of data it contains.  An object's type is fixed when it is created.
Types themselves are represented as objects; an object contains a
pointer to the corresponding type object.  The type itself has a type
pointer pointing to the object representing the type 'type', which
contains a pointer to itself!).

Objects do not float around in memory; once allocated an object keeps
the same size and address.  Objects that must hold variable-size data
can contain pointers to variable-size parts of the object.  Not all
objects of the same type have the same size; but the size cannot change
after allocation.  (These restrictions are made so a reference to an
object can be simply a pointer -- moving an object would require
updating all the pointers, and changing an object's size would require
moving it if there was another object right next to it.)

Objects are always accessed through pointers of the type 'PyObject *'.
The type 'PyObject' is a structure that only contains the reference count
and the type pointer.  The actual memory allocated for an object
contains other data that can only be accessed after casting the pointer
to a pointer to a longer structure type.  This longer type must start
with the reference count and type fields; the macro PyObject_HEAD should be
used for this (to accommodate for future changes).  The implementation
of a particular object type can cast the object pointer to the proper
type and back.

A standard interface exists for objects that contain an array of items
whose size is determined when the object is allocated.
*/
/* Py_DEBUG implies Py_TRACE_REFS. */
/* Py_TRACE_REFS implies Py_REF_DEBUG. */
/* PyObject_HEAD defines the initial segment of every PyObject. */
/* PyObject_VAR_HEAD defines the initial segment of all variable-size
 * container objects.  These end with a declaration of an array with 1
 * element, but enough space is malloc'ed so that the array actually
 * has room for ob_size elements.  Note that ob_size is an element count,
 * not necessarily a byte count.
 */
/* Number of items in variable part */
/* Nothing is actually declared to be a PyObject, but every pointer to
 * a Python object can be cast to a PyObject*.  This is inheritance built
 * by hand.  Similarly every pointer to a variable-size Python object can,
 * in addition, be cast to PyVarObject*.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _object {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _typeobject {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub ob_size: Py_ssize_t,
    pub tp_name: *const libc::c_char,
    pub tp_basicsize: Py_ssize_t,
    pub tp_itemsize: Py_ssize_t,
    pub tp_dealloc: destructor,
    pub tp_print: printfunc,
    pub tp_getattr: getattrfunc,
    pub tp_setattr: setattrfunc,
    pub tp_compare: cmpfunc,
    pub tp_repr: reprfunc,
    pub tp_as_number: *mut PyNumberMethods,
    pub tp_as_sequence: *mut PySequenceMethods,
    pub tp_as_mapping: *mut PyMappingMethods,
    pub tp_hash: hashfunc,
    pub tp_call: ternaryfunc,
    pub tp_str: reprfunc,
    pub tp_getattro: getattrofunc,
    pub tp_setattro: setattrofunc,
    pub tp_as_buffer: *mut PyBufferProcs,
    pub tp_flags: libc::c_long,
    pub tp_doc: *const libc::c_char,
    pub tp_traverse: traverseproc,
    pub tp_clear: inquiry,
    pub tp_richcompare: richcmpfunc,
    pub tp_weaklistoffset: Py_ssize_t,
    pub tp_iter: getiterfunc,
    pub tp_iternext: iternextfunc,
    pub tp_methods: *mut PyMethodDef,
    pub tp_members: *mut PyMemberDef,
    pub tp_getset: *mut PyGetSetDef,
    pub tp_base: *mut _typeobject,
    pub tp_dict: *mut PyObject,
    pub tp_descr_get: descrgetfunc,
    pub tp_descr_set: descrsetfunc,
    pub tp_dictoffset: Py_ssize_t,
    pub tp_init: initproc,
    pub tp_alloc: allocfunc,
    pub tp_new: newfunc,
    pub tp_free: freefunc,
    pub tp_is_gc: inquiry,
    pub tp_bases: *mut PyObject,
    pub tp_mro: *mut PyObject,
    pub tp_cache: *mut PyObject,
    pub tp_subclasses: *mut PyObject,
    pub tp_weaklist: *mut PyObject,
    pub tp_del: destructor,
    pub tp_version_tag: libc::c_uint,
}
/*
Type objects contain a string containing the type name (to help somewhat
in debugging), the allocation parameters (see PyObject_New() and
PyObject_NewVar()),
and methods for accessing objects of the type.  Methods are optional, a
nil pointer meaning that particular kind of access is not available for
this type.  The Py_DECREF() macro uses the tp_dealloc method without
checking for a nil pointer; it should always be implemented except if
the implementation can guarantee that the reference count will never
reach zero (e.g., for statically allocated type objects).

NB: the methods for certain type groups are now contained in separate
method blocks.
*/
/* int-based buffer interface */
/* ssize_t-based buffer interface */
/* Py3k buffer interface */
/* owned reference */
/* This is Py_ssize_t so it can be
                             pointed to by strides in simple case.*/
/* static store for shape and strides of
                                  mono-dimensional buffers. */
/* Flags for getting buffers */
/*  we used to include an E, backwards compatible alias  */
/* end Py3k buffer interface */
/* For numbers without flag bit Py_TPFLAGS_CHECKTYPES set, all
       arguments are guaranteed to be of the object's type (modulo
       coercion hacks -- i.e. if the type's coercion function
       returns other types, then these are allowed as well).  Numbers that
       have the Py_TPFLAGS_CHECKTYPES flag bit set should check *both*
       arguments for proper type and implement the necessary conversions
       in the slot functions themselves. */
/* Added in release 2.0 */
/* Added in release 2.2 */
    /* The following require the Py_TPFLAGS_HAVE_CLASS flag */
/* Added in release 2.5 */
/* Added in release 2.0 */
pub type destructor = Option<unsafe extern "C" fn(_: *mut PyObject) -> ()>;
pub type PyObject = _object;
pub type inquiry
    =
    Option<unsafe extern "C" fn(_: *mut PyObject) -> libc::c_int>;
pub type freefunc = Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub type newfunc
    =
    Option<unsafe extern "C" fn(_: *mut _typeobject, _: *mut PyObject,
                                _: *mut PyObject) -> *mut PyObject>;
pub type allocfunc
    =
    Option<unsafe extern "C" fn(_: *mut _typeobject, _: Py_ssize_t)
               -> *mut PyObject>;
pub type initproc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject,
                                _: *mut PyObject) -> libc::c_int>;
pub type descrsetfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject,
                                _: *mut PyObject) -> libc::c_int>;
pub type descrgetfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject,
                                _: *mut PyObject) -> *mut PyObject>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyGetSetDef {
    pub name: *mut libc::c_char,
    pub get: getter,
    pub set: setter,
    pub doc: *mut libc::c_char,
    pub closure: *mut libc::c_void,
}
/* Descriptors */
pub type setter
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject,
                                _: *mut libc::c_void) -> libc::c_int>;
pub type getter
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut libc::c_void)
               -> *mut PyObject>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyMethodDef {
    pub ml_name: *const libc::c_char,
    pub ml_meth: PyCFunction,
    pub ml_flags: libc::c_int,
    pub ml_doc: *const libc::c_char,
}
/* Method object interface */
/* This is about the type 'builtin_function_or_method',
   not Python methods in user-defined classes.  See classobject.h
   for the latter. */
pub type PyCFunction
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject)
               -> *mut PyObject>;
pub type iternextfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject) -> *mut PyObject>;
pub type getiterfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject) -> *mut PyObject>;
pub type richcmpfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject,
                                _: libc::c_int) -> *mut PyObject>;
pub type traverseproc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: visitproc,
                                _: *mut libc::c_void) -> libc::c_int>;
pub type visitproc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut libc::c_void)
               -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyBufferProcs {
    pub bf_getreadbuffer: readbufferproc,
    pub bf_getwritebuffer: writebufferproc,
    pub bf_getsegcount: segcountproc,
    pub bf_getcharbuffer: charbufferproc,
    pub bf_getbuffer: getbufferproc,
    pub bf_releasebuffer: releasebufferproc,
}
pub type releasebufferproc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut Py_buffer) -> ()>;
pub type Py_buffer = bufferinfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bufferinfo {
    pub buf: *mut libc::c_void,
    pub obj: *mut PyObject,
    pub len: Py_ssize_t,
    pub itemsize: Py_ssize_t,
    pub readonly: libc::c_int,
    pub ndim: libc::c_int,
    pub format: *mut libc::c_char,
    pub shape: *mut Py_ssize_t,
    pub strides: *mut Py_ssize_t,
    pub suboffsets: *mut Py_ssize_t,
    pub smalltable: [Py_ssize_t; 2],
    pub internal: *mut libc::c_void,
}
pub type getbufferproc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut Py_buffer,
                                _: libc::c_int) -> libc::c_int>;
pub type charbufferproc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: Py_ssize_t,
                                _: *mut *mut libc::c_char) -> Py_ssize_t>;
pub type segcountproc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut Py_ssize_t)
               -> Py_ssize_t>;
pub type writebufferproc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: Py_ssize_t,
                                _: *mut *mut libc::c_void) -> Py_ssize_t>;
pub type readbufferproc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: Py_ssize_t,
                                _: *mut *mut libc::c_void) -> Py_ssize_t>;
pub type setattrofunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject,
                                _: *mut PyObject) -> libc::c_int>;
pub type getattrofunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject)
               -> *mut PyObject>;
pub type reprfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject) -> *mut PyObject>;
pub type ternaryfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject,
                                _: *mut PyObject) -> *mut PyObject>;
pub type hashfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject) -> libc::c_long>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyMappingMethods {
    pub mp_length: lenfunc,
    pub mp_subscript: binaryfunc,
    pub mp_ass_subscript: objobjargproc,
}
pub type objobjargproc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject,
                                _: *mut PyObject) -> libc::c_int>;
pub type binaryfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject)
               -> *mut PyObject>;
pub type lenfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject) -> Py_ssize_t>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PySequenceMethods {
    pub sq_length: lenfunc,
    pub sq_concat: binaryfunc,
    pub sq_repeat: ssizeargfunc,
    pub sq_item: ssizeargfunc,
    pub sq_slice: ssizessizeargfunc,
    pub sq_ass_item: ssizeobjargproc,
    pub sq_ass_slice: ssizessizeobjargproc,
    pub sq_contains: objobjproc,
    pub sq_inplace_concat: binaryfunc,
    pub sq_inplace_repeat: ssizeargfunc,
}
pub type ssizeargfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: Py_ssize_t)
               -> *mut PyObject>;
pub type objobjproc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject)
               -> libc::c_int>;
pub type ssizessizeobjargproc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: Py_ssize_t,
                                _: Py_ssize_t, _: *mut PyObject)
               -> libc::c_int>;
pub type ssizeobjargproc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: Py_ssize_t,
                                _: *mut PyObject) -> libc::c_int>;
pub type ssizessizeargfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: Py_ssize_t,
                                _: Py_ssize_t) -> *mut PyObject>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyNumberMethods {
    pub nb_add: binaryfunc,
    pub nb_subtract: binaryfunc,
    pub nb_multiply: binaryfunc,
    pub nb_divide: binaryfunc,
    pub nb_remainder: binaryfunc,
    pub nb_divmod: binaryfunc,
    pub nb_power: ternaryfunc,
    pub nb_negative: unaryfunc,
    pub nb_positive: unaryfunc,
    pub nb_absolute: unaryfunc,
    pub nb_nonzero: inquiry,
    pub nb_invert: unaryfunc,
    pub nb_lshift: binaryfunc,
    pub nb_rshift: binaryfunc,
    pub nb_and: binaryfunc,
    pub nb_xor: binaryfunc,
    pub nb_or: binaryfunc,
    pub nb_coerce: coercion,
    pub nb_int: unaryfunc,
    pub nb_long: unaryfunc,
    pub nb_float: unaryfunc,
    pub nb_oct: unaryfunc,
    pub nb_hex: unaryfunc,
    pub nb_inplace_add: binaryfunc,
    pub nb_inplace_subtract: binaryfunc,
    pub nb_inplace_multiply: binaryfunc,
    pub nb_inplace_divide: binaryfunc,
    pub nb_inplace_remainder: binaryfunc,
    pub nb_inplace_power: ternaryfunc,
    pub nb_inplace_lshift: binaryfunc,
    pub nb_inplace_rshift: binaryfunc,
    pub nb_inplace_and: binaryfunc,
    pub nb_inplace_xor: binaryfunc,
    pub nb_inplace_or: binaryfunc,
    pub nb_floor_divide: binaryfunc,
    pub nb_true_divide: binaryfunc,
    pub nb_inplace_floor_divide: binaryfunc,
    pub nb_inplace_true_divide: binaryfunc,
    pub nb_index: unaryfunc,
}
pub type unaryfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject) -> *mut PyObject>;
pub type coercion
    =
    Option<unsafe extern "C" fn(_: *mut *mut PyObject, _: *mut *mut PyObject)
               -> libc::c_int>;
pub type cmpfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject)
               -> libc::c_int>;
pub type setattrfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut libc::c_char,
                                _: *mut PyObject) -> libc::c_int>;
pub type getattrfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut libc::c_char)
               -> *mut PyObject>;
pub type printfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut FILE,
                                _: libc::c_int) -> libc::c_int>;
pub type PyTypeObject = _typeobject;
/* Integer object interface */
/*
PyIntObject represents a (long) integer.  This is an immutable object;
an integer cannot change its value after creation.

There are functions to create new integer objects, to test an object
for integer-ness, and to get the integer value.  The latter functions
returns -1 and sets errno to EBADF if the object is not an PyIntObject.
None of the functions should be applied to nil objects.

The type PyIntObject is (unfortunately) exposed here so we can declare
_Py_TrueStruct and _Py_ZeroStruct in boolobject.h; don't use this.
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyIntObject {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub ob_ival: libc::c_long,
}
/*------------------------------------------------------------------------
 *  Copyright 2007-2010 (c) Jeff Brown <spadix@users.sourceforge.net>
 *
 *  This file is part of the ZBar Bar Code Reader.
 *
 *  The ZBar Bar Code Reader is free software; you can redistribute it
 *  and/or modify it under the terms of the GNU Lesser Public License as
 *  published by the Free Software Foundation; either version 2.1 of
 *  the License, or (at your option) any later version.
 *
 *  The ZBar Bar Code Reader is distributed in the hope that it will be
 *  useful, but WITHOUT ANY WARRANTY; without even the implied warranty
 *  of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU Lesser Public License for more details.
 *
 *  You should have received a copy of the GNU Lesser Public License
 *  along with the ZBar Bar Code Reader; if not, write to the Free
 *  Software Foundation, Inc., 51 Franklin St, Fifth Floor,
 *  Boston, MA  02110-1301  USA
 *
 *  http://sourceforge.net/projects/zbar
 *------------------------------------------------------------------------*/
/* * @file
 * ZBar Barcode Reader C API definition
 */
/* * @mainpage
 *
 * interface to the barcode reader is available at several levels.
 * most applications will want to use the high-level interfaces:
 *
 * @section high-level High-Level Interfaces
 *
 * these interfaces wrap all library functionality into an easy-to-use
 * package for a specific toolkit:
 * - the "GTK+ 2.x widget" may be used with GTK GUI applications.  a
 *   Python wrapper is included for PyGtk
 * - the @ref zbar::QZBar "Qt4 widget" may be used with Qt GUI
 *   applications
 * - the Processor interface (in @ref c-processor "C" or @ref
 *   zbar::Processor "C++") adds a scanning window to an application
 *   with no GUI.
 *
 * @section mid-level Intermediate Interfaces
 *
 * building blocks used to construct high-level interfaces:
 * - the ImageScanner (in @ref c-imagescanner "C" or @ref
 *   zbar::ImageScanner "C++") looks for barcodes in a library defined
 *   image object
 * - the Window abstraction (in @ref c-window "C" or @ref
 *   zbar::Window "C++") sinks library images, displaying them on the
 *   platform display
 * - the Video abstraction (in @ref c-video "C" or @ref zbar::Video
 *   "C++") sources library images from a video device
 *
 * @section low-level Low-Level Interfaces
 *
 * direct interaction with barcode scanning and decoding:
 * - the Scanner (in @ref c-scanner "C" or @ref zbar::Scanner "C++")
 *   looks for barcodes in a linear intensity sample stream
 * - the Decoder (in @ref c-decoder "C" or @ref zbar::Decoder "C++")
 *   extracts barcodes from a stream of bar and space widths
 */
/* * @name Global library interfaces */
/*@{*/
/* * "color" of element: bar or space. */
pub type zbar_color_e = libc::c_uint;
/* *< dark area or colored bar segment */
/* *< light area or space between bars */
pub const ZBAR_BAR: zbar_color_e = 1;
pub const ZBAR_SPACE: zbar_color_e = 0;
/* * decoded symbol type. */
pub type zbar_symbol_type_e = libc::c_uint;
/* * add-on flag mask.
     * @deprecated in 0.11, GS1 add-ons are represented using composite
     * symbols of type ::ZBAR_COMPOSITE; add-on components use ::ZBAR_EAN2
     * or ::ZBAR_EAN5
     */
pub const ZBAR_ADDON: zbar_symbol_type_e = 1792;
/* * 5-digit add-on flag.
     * @deprecated in 0.11, a ::ZBAR_EAN5 component is used for
     * 5-digit GS1 add-ons
     */
pub const ZBAR_ADDON5: zbar_symbol_type_e = 1280;
/* * 2-digit add-on flag.
     * @deprecated in 0.11, a ::ZBAR_EAN2 component is used for
     * 2-digit GS1 add-ons
     */
pub const ZBAR_ADDON2: zbar_symbol_type_e = 512;
/* *< Code 128 */
/* * mask for base symbol type.
     * @deprecated in 0.11, remove this from existing code
     */
pub const ZBAR_SYMBOL: zbar_symbol_type_e = 255;
/* *< Code 93. @since 0.11 */
pub const ZBAR_CODE128: zbar_symbol_type_e = 128;
/* *< QR Code. @since 0.10 */
pub const ZBAR_CODE93: zbar_symbol_type_e = 93;
/* *< PDF417. @since 0.6 */
pub const ZBAR_QRCODE: zbar_symbol_type_e = 64;
/* *< Code 39. @since 0.4 */
pub const ZBAR_PDF417: zbar_symbol_type_e = 57;
/* *< Codabar. @since 0.11 */
pub const ZBAR_CODE39: zbar_symbol_type_e = 39;
/* *< GS1 DataBar Expanded. @since 0.11 */
pub const ZBAR_CODABAR: zbar_symbol_type_e = 38;
/* *< GS1 DataBar (RSS). @since 0.11 */
pub const ZBAR_DATABAR_EXP: zbar_symbol_type_e = 35;
/* *< Interleaved 2 of 5. @since 0.4 */
pub const ZBAR_DATABAR: zbar_symbol_type_e = 34;
/* *< EAN/UPC composite */
pub const ZBAR_I25: zbar_symbol_type_e = 25;
/* *< ISBN-13 (from EAN-13). @since 0.4 */
pub const ZBAR_COMPOSITE: zbar_symbol_type_e = 15;
/* *< EAN-13 */
pub const ZBAR_ISBN13: zbar_symbol_type_e = 14;
/* *< UPC-A */
pub const ZBAR_EAN13: zbar_symbol_type_e = 13;
/* *< ISBN-10 (from EAN-13). @since 0.4 */
pub const ZBAR_UPCA: zbar_symbol_type_e = 12;
/* *< UPC-E */
pub const ZBAR_ISBN10: zbar_symbol_type_e = 10;
/* *< EAN-8 */
pub const ZBAR_UPCE: zbar_symbol_type_e = 9;
/* *< GS1 5-digit add-on */
pub const ZBAR_EAN8: zbar_symbol_type_e = 8;
/* *< GS1 2-digit add-on */
pub const ZBAR_EAN5: zbar_symbol_type_e = 5;
/* *< intermediate status */
pub const ZBAR_EAN2: zbar_symbol_type_e = 2;
/* *< no symbol decoded */
pub const ZBAR_PARTIAL: zbar_symbol_type_e = 1;
pub const ZBAR_NONE: zbar_symbol_type_e = 0;
pub type zbar_symbol_type_t = zbar_symbol_type_e;
/* * decoded symbol coarse orientation.
 * @since 0.11
 */
pub type zbar_orientation_e = libc::c_int;
/* *< sideways, read bottom to top */
/* *< upside-down, read right to left */
pub const ZBAR_ORIENT_LEFT: zbar_orientation_e = 3;
/* *< sideways, read top to bottom */
pub const ZBAR_ORIENT_DOWN: zbar_orientation_e = 2;
/* *< upright, read left to right */
pub const ZBAR_ORIENT_RIGHT: zbar_orientation_e = 1;
/* *< unable to determine orientation */
pub const ZBAR_ORIENT_UP: zbar_orientation_e = 0;
pub const ZBAR_ORIENT_UNKNOWN: zbar_orientation_e = -1;
/* * error codes. */
pub type zbar_error_e = libc::c_uint;
/* *< number of error codes */
/* *< windows system error */
pub const ZBAR_ERR_NUM: zbar_error_e = 12;
/* *< output window is closed */
pub const ZBAR_ERR_WINAPI: zbar_error_e = 11;
/* *< X11 protocol error */
pub const ZBAR_ERR_CLOSED: zbar_error_e = 10;
/* *< X11 display error */
pub const ZBAR_ERR_XPROTO: zbar_error_e = 9;
/* *< all resources busy */
pub const ZBAR_ERR_XDISPLAY: zbar_error_e = 8;
/* *< locking error */
pub const ZBAR_ERR_BUSY: zbar_error_e = 7;
/* *< system error */
pub const ZBAR_ERR_LOCKING: zbar_error_e = 6;
/* *< invalid request */
pub const ZBAR_ERR_SYSTEM: zbar_error_e = 5;
/* *< unsupported request */
pub const ZBAR_ERR_INVALID: zbar_error_e = 4;
/* *< internal library error */
pub const ZBAR_ERR_UNSUPPORTED: zbar_error_e = 3;
/* *< out of memory */
pub const ZBAR_ERR_INTERNAL: zbar_error_e = 2;
/* *< no error */
pub const ZBAR_ERR_NOMEM: zbar_error_e = 1;
pub const ZBAR_OK: zbar_error_e = 0;
pub type zbar_error_t = zbar_error_e;
/* * decoder configuration options.
 * @since 0.4
 */
pub type zbar_config_e = libc::c_uint;
/* *< image scanner horizontal scan density */
/* *< image scanner vertical scan density */
pub const ZBAR_CFG_Y_DENSITY: zbar_config_e = 257;
/* *< enable scanner to collect position data */
pub const ZBAR_CFG_X_DENSITY: zbar_config_e = 256;
/* *< required video consistency frames */
pub const ZBAR_CFG_POSITION: zbar_config_e = 128;
/* *< maximum data length for valid decode */
pub const ZBAR_CFG_UNCERTAINTY: zbar_config_e = 64;
/* *< minimum data length for valid decode */
pub const ZBAR_CFG_MAX_LEN: zbar_config_e = 33;
/* *< number of boolean decoder configs */
pub const ZBAR_CFG_MIN_LEN: zbar_config_e = 32;
/* *< enable full ASCII character set */
pub const ZBAR_CFG_NUM: zbar_config_e = 4;
/* *< return check digit when present */
pub const ZBAR_CFG_ASCII: zbar_config_e = 3;
/* *< enable check digit when optional */
pub const ZBAR_CFG_EMIT_CHECK: zbar_config_e = 2;
/* *< enable symbology/feature */
pub const ZBAR_CFG_ADD_CHECK: zbar_config_e = 1;
pub const ZBAR_CFG_ENABLE: zbar_config_e = 0;
/* * decoder symbology modifier flags.
 * @since 0.11
 */
pub type zbar_modifier_e = libc::c_uint;
/* * number of modifiers */
pub const ZBAR_MOD_NUM: zbar_modifier_e = 2;
/* * barcode tagged as AIM reserved
     * (eg, FNC1 after first character or digit pair)
     */
pub const ZBAR_MOD_AIM: zbar_modifier_e = 1;
/* * barcode tagged as GS1 (EAN.UCC) reserved
     * (eg, FNC1 before first data character).
     * data may be parsed as a sequence of GS1 AIs
     */
pub const ZBAR_MOD_GS1: zbar_modifier_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zbarEnumItem {
    pub val: PyIntObject,
    pub name: *mut PyObject,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zbarEnum {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub byname: *mut PyObject,
    pub byvalue: *mut PyObject,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct enumdef {
    pub strval: *const libc::c_char,
    pub intval: libc::c_int,
}
static mut exc_names: [*mut libc::c_char; 12] =
    [b"zbar.Exception\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char, 0 as *const libc::c_char as *mut libc::c_char,
     b"zbar.InternalError\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"zbar.UnsupportedError\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"zbar.InvalidRequestError\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"zbar.SystemError\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"zbar.LockingError\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"zbar.BusyError\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"zbar.X11DisplayError\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"zbar.X11ProtocolError\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"zbar.WindowClosed\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"zbar.WinAPIError\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char];
static mut symbol_defs: [enumdef; 18] =
    [{
         let mut init =
             enumdef{strval: b"NONE\x00" as *const u8 as *const libc::c_char,
                     intval: ZBAR_NONE as libc::c_int,};
         init
     },
     {
         let mut init =
             enumdef{strval:
                         b"PARTIAL\x00" as *const u8 as *const libc::c_char,
                     intval: ZBAR_PARTIAL as libc::c_int,};
         init
     },
     {
         let mut init =
             enumdef{strval: b"EAN8\x00" as *const u8 as *const libc::c_char,
                     intval: ZBAR_EAN8 as libc::c_int,};
         init
     },
     {
         let mut init =
             enumdef{strval: b"UPCE\x00" as *const u8 as *const libc::c_char,
                     intval: ZBAR_UPCE as libc::c_int,};
         init
     },
     {
         let mut init =
             enumdef{strval:
                         b"ISBN10\x00" as *const u8 as *const libc::c_char,
                     intval: ZBAR_ISBN10 as libc::c_int,};
         init
     },
     {
         let mut init =
             enumdef{strval: b"UPCA\x00" as *const u8 as *const libc::c_char,
                     intval: ZBAR_UPCA as libc::c_int,};
         init
     },
     {
         let mut init =
             enumdef{strval: b"EAN13\x00" as *const u8 as *const libc::c_char,
                     intval: ZBAR_EAN13 as libc::c_int,};
         init
     },
     {
         let mut init =
             enumdef{strval:
                         b"ISBN13\x00" as *const u8 as *const libc::c_char,
                     intval: ZBAR_ISBN13 as libc::c_int,};
         init
     },
     {
         let mut init =
             enumdef{strval:
                         b"DATABAR\x00" as *const u8 as *const libc::c_char,
                     intval: ZBAR_DATABAR as libc::c_int,};
         init
     },
     {
         let mut init =
             enumdef{strval:
                         b"DATABAR_EXP\x00" as *const u8 as
                             *const libc::c_char,
                     intval: ZBAR_DATABAR_EXP as libc::c_int,};
         init
     },
     {
         let mut init =
             enumdef{strval: b"I25\x00" as *const u8 as *const libc::c_char,
                     intval: ZBAR_I25 as libc::c_int,};
         init
     },
     {
         let mut init =
             enumdef{strval:
                         b"CODABAR\x00" as *const u8 as *const libc::c_char,
                     intval: ZBAR_CODABAR as libc::c_int,};
         init
     },
     {
         let mut init =
             enumdef{strval:
                         b"CODE39\x00" as *const u8 as *const libc::c_char,
                     intval: ZBAR_CODE39 as libc::c_int,};
         init
     },
     {
         let mut init =
             enumdef{strval:
                         b"PDF417\x00" as *const u8 as *const libc::c_char,
                     intval: ZBAR_PDF417 as libc::c_int,};
         init
     },
     {
         let mut init =
             enumdef{strval:
                         b"QRCODE\x00" as *const u8 as *const libc::c_char,
                     intval: ZBAR_QRCODE as libc::c_int,};
         init
     },
     {
         let mut init =
             enumdef{strval:
                         b"CODE93\x00" as *const u8 as *const libc::c_char,
                     intval: ZBAR_CODE93 as libc::c_int,};
         init
     },
     {
         let mut init =
             enumdef{strval:
                         b"CODE128\x00" as *const u8 as *const libc::c_char,
                     intval: ZBAR_CODE128 as libc::c_int,};
         init
     },
     {
         let mut init = enumdef{strval: 0 as *const libc::c_char, intval: 0,};
         init
     }];
static mut config_defs: [enumdef; 11] =
    [{
         let mut init =
             enumdef{strval:
                         b"ENABLE\x00" as *const u8 as *const libc::c_char,
                     intval: ZBAR_CFG_ENABLE as libc::c_int,};
         init
     },
     {
         let mut init =
             enumdef{strval:
                         b"ADD_CHECK\x00" as *const u8 as *const libc::c_char,
                     intval: ZBAR_CFG_ADD_CHECK as libc::c_int,};
         init
     },
     {
         let mut init =
             enumdef{strval:
                         b"EMIT_CHECK\x00" as *const u8 as
                             *const libc::c_char,
                     intval: ZBAR_CFG_EMIT_CHECK as libc::c_int,};
         init
     },
     {
         let mut init =
             enumdef{strval: b"ASCII\x00" as *const u8 as *const libc::c_char,
                     intval: ZBAR_CFG_ASCII as libc::c_int,};
         init
     },
     {
         let mut init =
             enumdef{strval:
                         b"MIN_LEN\x00" as *const u8 as *const libc::c_char,
                     intval: ZBAR_CFG_MIN_LEN as libc::c_int,};
         init
     },
     {
         let mut init =
             enumdef{strval:
                         b"MAX_LEN\x00" as *const u8 as *const libc::c_char,
                     intval: ZBAR_CFG_MAX_LEN as libc::c_int,};
         init
     },
     {
         let mut init =
             enumdef{strval:
                         b"UNCERTAINTY\x00" as *const u8 as
                             *const libc::c_char,
                     intval: ZBAR_CFG_UNCERTAINTY as libc::c_int,};
         init
     },
     {
         let mut init =
             enumdef{strval:
                         b"POSITION\x00" as *const u8 as *const libc::c_char,
                     intval: ZBAR_CFG_POSITION as libc::c_int,};
         init
     },
     {
         let mut init =
             enumdef{strval:
                         b"X_DENSITY\x00" as *const u8 as *const libc::c_char,
                     intval: ZBAR_CFG_X_DENSITY as libc::c_int,};
         init
     },
     {
         let mut init =
             enumdef{strval:
                         b"Y_DENSITY\x00" as *const u8 as *const libc::c_char,
                     intval: ZBAR_CFG_Y_DENSITY as libc::c_int,};
         init
     },
     {
         let mut init = enumdef{strval: 0 as *const libc::c_char, intval: 0,};
         init
     }];
static mut modifier_defs: [enumdef; 3] =
    [{
         let mut init =
             enumdef{strval: b"GS1\x00" as *const u8 as *const libc::c_char,
                     intval: ZBAR_MOD_GS1 as libc::c_int,};
         init
     },
     {
         let mut init =
             enumdef{strval: b"AIM\x00" as *const u8 as *const libc::c_char,
                     intval: ZBAR_MOD_AIM as libc::c_int,};
         init
     },
     {
         let mut init = enumdef{strval: 0 as *const libc::c_char, intval: 0,};
         init
     }];
static mut orient_defs: [enumdef; 6] =
    [{
         let mut init =
             enumdef{strval:
                         b"UNKNOWN\x00" as *const u8 as *const libc::c_char,
                     intval: ZBAR_ORIENT_UNKNOWN as libc::c_int,};
         init
     },
     {
         let mut init =
             enumdef{strval: b"UP\x00" as *const u8 as *const libc::c_char,
                     intval: ZBAR_ORIENT_UP as libc::c_int,};
         init
     },
     {
         let mut init =
             enumdef{strval: b"RIGHT\x00" as *const u8 as *const libc::c_char,
                     intval: ZBAR_ORIENT_RIGHT as libc::c_int,};
         init
     },
     {
         let mut init =
             enumdef{strval: b"DOWN\x00" as *const u8 as *const libc::c_char,
                     intval: ZBAR_ORIENT_DOWN as libc::c_int,};
         init
     },
     {
         let mut init =
             enumdef{strval: b"LEFT\x00" as *const u8 as *const libc::c_char,
                     intval: ZBAR_ORIENT_LEFT as libc::c_int,};
         init
     },
     {
         let mut init = enumdef{strval: 0 as *const libc::c_char, intval: 0,};
         init
     }];
#[no_mangle]
pub unsafe extern "C" fn object_to_bool(mut obj: *mut PyObject,
                                        mut val: *mut libc::c_int)
 -> libc::c_int {
    let mut tmp: libc::c_int = PyObject_IsTrue(obj);
    if tmp < 0 as libc::c_int { return 0 as libc::c_int }
    *val = tmp;
    return 1 as libc::c_int;
}
/*------------------------------------------------------------------------
 *  Copyright 2009-2010 (c) Jeff Brown <spadix@users.sourceforge.net>
 *
 *  This file is part of the ZBar Bar Code Reader.
 *
 *  The ZBar Bar Code Reader is free software; you can redistribute it
 *  and/or modify it under the terms of the GNU Lesser Public License as
 *  published by the Free Software Foundation; either version 2.1 of
 *  the License, or (at your option) any later version.
 *
 *  The ZBar Bar Code Reader is distributed in the hope that it will be
 *  useful, but WITHOUT ANY WARRANTY; without even the implied warranty
 *  of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU Lesser Public License for more details.
 *
 *  You should have received a copy of the GNU Lesser Public License
 *  along with the ZBar Bar Code Reader; if not, write to the Free
 *  Software Foundation, Inc., 51 Franklin St, Fifth Floor,
 *  Boston, MA  02110-1301  USA
 *
 *  http://sourceforge.net/projects/zbar
 *------------------------------------------------------------------------*/
/* integer value is super type */
/* associated string name */
/* zbarEnumItem content dictionaries */
#[no_mangle]
pub unsafe extern "C" fn parse_dimensions(mut seq: *mut PyObject,
                                          mut dims: *mut libc::c_int,
                                          mut n: libc::c_int) -> libc::c_int {
    if PySequence_Check(seq) == 0 || PySequence_Size(seq) != n as libc::c_long
       {
        return -(1 as libc::c_int)
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        let mut dim: *mut PyObject = PySequence_GetItem(seq, i as Py_ssize_t);
        if dim.is_null() { return -(1 as libc::c_int) }
        *dims = PyInt_AsSsize_t(dim) as libc::c_int;
        (*dim).ob_refcnt -= 1;
        if !((*dim).ob_refcnt != 0 as libc::c_int as libc::c_long) {
            Some((*(*dim).ob_type).tp_dealloc.expect("non-null function pointer")).expect("non-null function pointer")(dim);
        }
        if *dims == -(1 as libc::c_int) && !PyErr_Occurred().is_null() {
            return -(1 as libc::c_int)
        }
        i += 1;
        dims = dims.offset(1)
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub static mut zbar_exc: [*mut PyObject; 12] =
    [0 as *const PyObject as *mut PyObject; 12];
#[no_mangle]
pub static mut color_enum: [*mut zbarEnumItem; 2] =
    [0 as *const zbarEnumItem as *mut zbarEnumItem; 2];
#[no_mangle]
pub static mut config_enum: *mut zbarEnum =
    0 as *const zbarEnum as *mut zbarEnum;
#[no_mangle]
pub static mut modifier_enum: *mut zbarEnum =
    0 as *const zbarEnum as *mut zbarEnum;
#[no_mangle]
pub static mut symbol_enum: *mut PyObject =
    0 as *const PyObject as *mut PyObject;
#[no_mangle]
pub static mut symbol_NONE: *mut zbarEnumItem =
    0 as *const zbarEnumItem as *mut zbarEnumItem;
#[no_mangle]
pub static mut orient_enum: *mut zbarEnum =
    0 as *const zbarEnum as *mut zbarEnum;
unsafe extern "C" fn version(mut self_0: *mut PyObject,
                             mut args: *mut PyObject) -> *mut PyObject {
    if PyArg_ParseTuple(args, b"\x00" as *const u8 as *const libc::c_char) ==
           0 {
        return 0 as *mut PyObject
    }
    let mut major: libc::c_uint = 0;
    let mut minor: libc::c_uint = 0;
    zbar_version(&mut major, &mut minor);
    return Py_BuildValue(b"II\x00" as *const u8 as *const libc::c_char, major,
                         minor);
}
unsafe extern "C" fn set_verbosity(mut self_0: *mut PyObject,
                                   mut args: *mut PyObject) -> *mut PyObject {
    let mut verbosity: libc::c_int = 0;
    if PyArg_ParseTuple(args, b"i\x00" as *const u8 as *const libc::c_char,
                        &mut verbosity as *mut libc::c_int) == 0 {
        return 0 as *mut PyObject
    }
    zbar_set_verbosity(verbosity);
    let ref mut fresh0 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
    *fresh0 += 1;
    return &mut _Py_NoneStruct;
}
unsafe extern "C" fn increase_verbosity(mut self_0: *mut PyObject,
                                        mut args: *mut PyObject)
 -> *mut PyObject {
    if PyArg_ParseTuple(args, b"\x00" as *const u8 as *const libc::c_char) ==
           0 {
        return 0 as *mut PyObject
    }
    zbar_increase_verbosity();
    let ref mut fresh1 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
    *fresh1 += 1;
    return &mut _Py_NoneStruct;
}
static mut zbar_functions: [PyMethodDef; 4] =
    unsafe {
        [{
             let mut init =
                 PyMethodDef{ml_name:
                                 b"version\x00" as *const u8 as
                                     *const libc::c_char,
                             ml_meth:
                                 Some(version as
                                          unsafe extern "C" fn(_:
                                                                   *mut PyObject,
                                                               _:
                                                                   *mut PyObject)
                                              -> *mut PyObject),
                             ml_flags: 0x1 as libc::c_int,
                             ml_doc: 0 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 PyMethodDef{ml_name:
                                 b"set_verbosity\x00" as *const u8 as
                                     *const libc::c_char,
                             ml_meth:
                                 Some(set_verbosity as
                                          unsafe extern "C" fn(_:
                                                                   *mut PyObject,
                                                               _:
                                                                   *mut PyObject)
                                              -> *mut PyObject),
                             ml_flags: 0x1 as libc::c_int,
                             ml_doc: 0 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 PyMethodDef{ml_name:
                                 b"increase_verbosity\x00" as *const u8 as
                                     *const libc::c_char,
                             ml_meth:
                                 Some(increase_verbosity as
                                          unsafe extern "C" fn(_:
                                                                   *mut PyObject,
                                                               _:
                                                                   *mut PyObject)
                                              -> *mut PyObject),
                             ml_flags: 0x1 as libc::c_int,
                             ml_doc: 0 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 PyMethodDef{ml_name: 0 as *const libc::c_char,
                             ml_meth: None,
                             ml_flags: 0,
                             ml_doc: 0 as *const libc::c_char,};
             init
         }]
    };
#[no_mangle]
pub unsafe extern "C" fn initzbar() {
    /* initialize types */
    zbarEnumItem_Type.tp_base = &mut PyInt_Type;
    zbarException_Type.tp_base = PyExc_Exception as *mut PyTypeObject;
    if PyType_Ready(&mut zbarException_Type) < 0 as libc::c_int ||
           PyType_Ready(&mut zbarEnumItem_Type) < 0 as libc::c_int ||
           PyType_Ready(&mut zbarEnum_Type) < 0 as libc::c_int ||
           PyType_Ready(&mut zbarImage_Type) < 0 as libc::c_int ||
           PyType_Ready(&mut zbarSymbol_Type) < 0 as libc::c_int ||
           PyType_Ready(&mut zbarSymbolSet_Type) < 0 as libc::c_int ||
           PyType_Ready(&mut zbarSymbolIter_Type) < 0 as libc::c_int ||
           PyType_Ready(&mut zbarProcessor_Type) < 0 as libc::c_int ||
           PyType_Ready(&mut zbarImageScanner_Type) < 0 as libc::c_int ||
           PyType_Ready(&mut zbarDecoder_Type) < 0 as libc::c_int ||
           PyType_Ready(&mut zbarScanner_Type) < 0 as libc::c_int {
        return
    }
    /* initialize constant containers */
    config_enum = zbarEnum_New();
    modifier_enum = zbarEnum_New();
    symbol_enum = PyDict_New();
    orient_enum = zbarEnum_New();
    if config_enum.is_null() || modifier_enum.is_null() ||
           symbol_enum.is_null() || orient_enum.is_null() {
        return
    }
    zbar_exc[0 as libc::c_int as usize] =
        &mut zbarException_Type as *mut PyTypeObject as *mut PyObject;
    zbar_exc[ZBAR_ERR_NOMEM as libc::c_int as usize] = 0 as *mut PyObject;
    let mut ei: zbar_error_t = ZBAR_OK;
    ei = ZBAR_ERR_INTERNAL;
    while (ei as libc::c_uint) < ZBAR_ERR_NUM as libc::c_int as libc::c_uint {
        zbar_exc[ei as usize] =
            PyErr_NewException(exc_names[ei as usize],
                               zbar_exc[0 as libc::c_int as usize],
                               0 as *mut PyObject);
        if zbar_exc[ei as usize].is_null() { return }
        ei += 1
    }
    /* internally created/read-only type overrides */
    zbarEnum_Type.tp_new = None;
    zbarEnum_Type.tp_setattr = None;
    zbarEnum_Type.tp_setattro = None;
    /* initialize module */
    let mut mod_0: *mut PyObject =
        Py_InitModule4_64(b"zbar\x00" as *const u8 as *const libc::c_char,
                          zbar_functions.as_mut_ptr(),
                          0 as *mut libc::c_void as *mut libc::c_char,
                          0 as *mut libc::c_void as *mut PyObject,
                          1013 as libc::c_int);
    if mod_0.is_null() { return }
    /* add types to module */
    PyModule_AddObject(mod_0,
                       b"EnumItem\x00" as *const u8 as *const libc::c_char,
                       &mut zbarEnumItem_Type as *mut PyTypeObject as
                           *mut PyObject);
    PyModule_AddObject(mod_0,
                       b"Image\x00" as *const u8 as *const libc::c_char,
                       &mut zbarImage_Type as *mut PyTypeObject as
                           *mut PyObject);
    PyModule_AddObject(mod_0,
                       b"Config\x00" as *const u8 as *const libc::c_char,
                       config_enum as *mut PyObject);
    PyModule_AddObject(mod_0,
                       b"Modifier\x00" as *const u8 as *const libc::c_char,
                       modifier_enum as *mut PyObject);
    PyModule_AddObject(mod_0,
                       b"Orient\x00" as *const u8 as *const libc::c_char,
                       orient_enum as *mut PyObject);
    PyModule_AddObject(mod_0,
                       b"Symbol\x00" as *const u8 as *const libc::c_char,
                       &mut zbarSymbol_Type as *mut PyTypeObject as
                           *mut PyObject);
    PyModule_AddObject(mod_0,
                       b"SymbolSet\x00" as *const u8 as *const libc::c_char,
                       &mut zbarSymbolSet_Type as *mut PyTypeObject as
                           *mut PyObject);
    PyModule_AddObject(mod_0,
                       b"SymbolIter\x00" as *const u8 as *const libc::c_char,
                       &mut zbarSymbolIter_Type as *mut PyTypeObject as
                           *mut PyObject);
    PyModule_AddObject(mod_0,
                       b"Processor\x00" as *const u8 as *const libc::c_char,
                       &mut zbarProcessor_Type as *mut PyTypeObject as
                           *mut PyObject);
    PyModule_AddObject(mod_0,
                       b"ImageScanner\x00" as *const u8 as
                           *const libc::c_char,
                       &mut zbarImageScanner_Type as *mut PyTypeObject as
                           *mut PyObject);
    PyModule_AddObject(mod_0,
                       b"Decoder\x00" as *const u8 as *const libc::c_char,
                       &mut zbarDecoder_Type as *mut PyTypeObject as
                           *mut PyObject);
    PyModule_AddObject(mod_0,
                       b"Scanner\x00" as *const u8 as *const libc::c_char,
                       &mut zbarScanner_Type as *mut PyTypeObject as
                           *mut PyObject);
    ei = ZBAR_OK;
    while (ei as libc::c_uint) < ZBAR_ERR_NUM as libc::c_int as libc::c_uint {
        if !zbar_exc[ei as usize].is_null() {
            PyModule_AddObject(mod_0,
                               exc_names[ei as
                                             usize].offset(5 as libc::c_int as
                                                               isize),
                               zbar_exc[ei as usize]);
        }
        ei += 1
    }
    /* add constants */
    let mut dict: *mut PyObject = PyModule_GetDict(mod_0);
    color_enum[ZBAR_SPACE as libc::c_int as usize] =
        zbarEnumItem_New(dict, 0 as *mut PyObject, ZBAR_SPACE as libc::c_int,
                         b"SPACE\x00" as *const u8 as *const libc::c_char);
    color_enum[ZBAR_BAR as libc::c_int as usize] =
        zbarEnumItem_New(dict, 0 as *mut PyObject, ZBAR_BAR as libc::c_int,
                         b"BAR\x00" as *const u8 as *const libc::c_char);
    let mut item: *const enumdef = 0 as *const enumdef;
    item = config_defs.as_ptr();
    while !(*item).strval.is_null() {
        zbarEnum_Add(config_enum, (*item).intval, (*item).strval);
        item = item.offset(1)
    }
    item = modifier_defs.as_ptr();
    while !(*item).strval.is_null() {
        zbarEnum_Add(modifier_enum, (*item).intval, (*item).strval);
        item = item.offset(1)
    }
    item = orient_defs.as_ptr();
    while !(*item).strval.is_null() {
        zbarEnum_Add(orient_enum, (*item).intval, (*item).strval);
        item = item.offset(1)
    }
    let mut tp_dict: *mut PyObject = zbarSymbol_Type.tp_dict;
    item = symbol_defs.as_ptr();
    while !(*item).strval.is_null() {
        zbarEnumItem_New(tp_dict, symbol_enum, (*item).intval,
                         (*item).strval);
        item = item.offset(1)
    }
    symbol_NONE = zbarSymbol_LookupEnum(ZBAR_NONE);
}
