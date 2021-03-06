use ::libc;
extern {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type PyMemberDef;
    /* @} */
    pub type zbar_symbol_s;
    pub type zbar_symbol_set_s;
    /* @} */
    /* ------------------------------------------------------------ */
    /* * @name Image interface
     * stores image data samples along with associated format and size
     * metadata
     */
    /* @{ */
    pub type zbar_image_s;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    /* access macro to the members which are floating "behind" the object */
    /* Generic type check */
    #[no_mangle]
    fn PyType_IsSubtype(_: *mut PyTypeObject, _: *mut PyTypeObject) -> libc::c_int;
    /*
    _Py_NoneStruct is an object of undefined type which can be used in contexts
    where NULL (nil) is not suitable (since NULL often means 'error').

    Don't forget to apply Py_INCREF() when returning this value!!!
    */
    #[no_mangle]
    static mut _Py_NoneStruct: PyObject;
    #[no_mangle]
    fn _PyObject_GC_New(_: *mut PyTypeObject) -> *mut PyObject;
    #[no_mangle]
    fn PyInt_FromLong(_: libc::c_long) -> *mut PyObject;
    #[no_mangle]
    fn PyInt_AsSsize_t(_: *mut PyObject) -> Py_ssize_t;
    #[no_mangle]
    fn PyString_FromStringAndSize(_: *const libc::c_char, _: Py_ssize_t) -> *mut PyObject;
    /* Provides access to the internal data buffer and size of a string
    object or the default encoded version of a Unicode object. Passing
    NULL as *len parameter will force the string buffer to be
    0-terminated (passing a string with embedded NULL characters will
    cause an exception).  */
    #[no_mangle]
    fn PyString_AsStringAndSize(
        obj: *mut PyObject,
        s: *mut *mut libc::c_char,
        len: *mut Py_ssize_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn PyBuffer_FromMemory(ptr: *mut libc::c_void, size: Py_ssize_t) -> *mut PyObject;
    #[no_mangle]
    fn PyTuple_Pack(_: Py_ssize_t, _: ...) -> *mut PyObject;
    #[no_mangle]
    fn PyErr_SetString(_: *mut PyObject, _: *const libc::c_char);
    #[no_mangle]
    fn PyErr_Occurred() -> *mut PyObject;
    #[no_mangle]
    static mut PyExc_TypeError: *mut PyObject;
    #[no_mangle]
    static mut PyExc_ValueError: *mut PyObject;
    /* MS_WINDOWS */
    #[no_mangle]
    fn PyErr_Format(_: *mut PyObject, _: *const libc::c_char, _: ...) -> *mut PyObject;
    #[no_mangle]
    fn PyArg_ParseTupleAndKeywords(
        _: *mut PyObject,
        _: *mut PyObject,
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: ...
    ) -> libc::c_int;
    /* * new image constructor.
     * @returns a new image object with uninitialized data and format.
     * this image should be destroyed (using zbar_image_destroy()) as
     * soon as the application is finished with it
     */
    #[no_mangle]
    fn zbar_image_create() -> *mut zbar_image_t;
    /* * image destructor.  all images created by or returned to the
     * application should be destroyed using this function.  when an image
     * is destroyed, the associated data cleanup handler will be invoked
     * if available
     * @note make no assumptions about the image or the data buffer.
     * they may not be destroyed/cleaned immediately if the library
     * is still using them.  if necessary, use the cleanup handler hook
     * to keep track of image data buffers
     */
    #[no_mangle]
    fn zbar_image_destroy(image: *mut zbar_image_t);
    /* * image reference count manipulation.
     * increment the reference count when you store a new reference to the
     * image.  decrement when the reference is no longer used.  do not
     * refer to the image any longer once the count is decremented.
     * zbar_image_ref(image, -1) is the same as zbar_image_destroy(image)
     * @since 0.5
     */
    #[no_mangle]
    fn zbar_image_ref(image: *mut zbar_image_t, refs: libc::c_int);
    /* * image format conversion.  refer to the documentation for supported
     * image formats
     * @returns a @em new image with the sample data from the original image
     * converted to the requested format.  the original image is
     * unaffected.
     * @note the converted image size may be rounded (up) due to format
     * constraints
     */
    #[no_mangle]
    fn zbar_image_convert(image: *const zbar_image_t, format: libc::c_ulong) -> *mut zbar_image_t;
    /* * image format conversion with crop/pad.
     * if the requested size is larger than the image, the last row/column
     * are duplicated to cover the difference.  if the requested size is
     * smaller than the image, the extra rows/columns are dropped from the
     * right/bottom.
     * @returns a @em new image with the sample data from the original
     * image converted to the requested format and size.
     * @note the image is @em not scaled
     * @see zbar_image_convert()
     * @since 0.4
     */
    #[no_mangle]
    fn zbar_image_convert_resize(
        image: *const zbar_image_t,
        format: libc::c_ulong,
        width: libc::c_uint,
        height: libc::c_uint,
    ) -> *mut zbar_image_t;
    /* * retrieve the image format.
     * @returns the fourcc describing the format of the image sample data
     */
    #[no_mangle]
    fn zbar_image_get_format(image: *const zbar_image_t) -> libc::c_ulong;
    /* * retrieve a "sequence" (page/frame) number associated with this image.
     * @since 0.6
     */
    #[no_mangle]
    fn zbar_image_get_sequence(image: *const zbar_image_t) -> libc::c_uint;
    /* * retrieve the width of the image.
     * @returns the width in sample columns
     */
    #[no_mangle]
    fn zbar_image_get_width(image: *const zbar_image_t) -> libc::c_uint;
    /* * retrieve the height of the image.
     * @returns the height in sample rows
     */
    #[no_mangle]
    fn zbar_image_get_height(image: *const zbar_image_t) -> libc::c_uint;
    /* * retrieve both dimensions of the image.
     * fills in the width and height in samples
     */
    #[no_mangle]
    fn zbar_image_get_size(
        image: *const zbar_image_t,
        width: *mut libc::c_uint,
        height: *mut libc::c_uint,
    );
    /* * retrieve the crop rectangle.
     * fills in the image coordinates of the upper left corner and size
     * of an axis-aligned rectangular area of the image that will be scanned.
     * defaults to the full image
     * @since 0.11
     */
    #[no_mangle]
    fn zbar_image_get_crop(
        image: *const zbar_image_t,
        x: *mut libc::c_uint,
        y: *mut libc::c_uint,
        width: *mut libc::c_uint,
        height: *mut libc::c_uint,
    );
    /* * return the image sample data.  the returned data buffer is only
     * valid until zbar_image_destroy() is called
     */
    #[no_mangle]
    fn zbar_image_get_data(image: *const zbar_image_t) -> *const libc::c_void;
    /* * return the size of image data.
     * @since 0.6
     */
    #[no_mangle]
    fn zbar_image_get_data_length(img: *const zbar_image_t) -> libc::c_ulong;
    /* * retrieve the decoded results.
     * @returns the (possibly empty) set of decoded symbols
     * @returns NULL if the image has not been scanned
     * @since 0.10
     */
    #[no_mangle]
    fn zbar_image_get_symbols(image: *const zbar_image_t) -> *const zbar_symbol_set_t;
    /* * associate the specified symbol set with the image, replacing any
     * existing results.  use NULL to release the current results from the
     * image.
     * @see zbar_image_scanner_recycle_image()
     * @since 0.10
     */
    #[no_mangle]
    fn zbar_image_set_symbols(image: *mut zbar_image_t, symbols: *const zbar_symbol_set_t);
    /* * specify the fourcc image format code for image sample data.
     * refer to the documentation for supported formats.
     * @note this does not convert the data!
     * (see zbar_image_convert() for that)
     */
    #[no_mangle]
    fn zbar_image_set_format(image: *mut zbar_image_t, format: libc::c_ulong);
    /* * associate a "sequence" (page/frame) number with this image.
     * @since 0.6
     */
    #[no_mangle]
    fn zbar_image_set_sequence(image: *mut zbar_image_t, sequence_num: libc::c_uint);
    /* * specify the pixel size of the image.
     * @note this also resets the crop rectangle to the full image
     * (0, 0, width, height)
     * @note this does not affect the data!
     */
    #[no_mangle]
    fn zbar_image_set_size(image: *mut zbar_image_t, width: libc::c_uint, height: libc::c_uint);
    /* * specify a rectangular region of the image to scan.
     * the rectangle will be clipped to the image boundaries.
     * defaults to the full image specified by zbar_image_set_size()
     */
    #[no_mangle]
    fn zbar_image_set_crop(
        image: *mut zbar_image_t,
        x: libc::c_uint,
        y: libc::c_uint,
        width: libc::c_uint,
        height: libc::c_uint,
    );
    /* * specify image sample data.  when image data is no longer needed by
     * the library the specific data cleanup handler will be called
     * (unless NULL)
     * @note application image data will not be modified by the library
     */
    #[no_mangle]
    fn zbar_image_set_data(
        image: *mut zbar_image_t,
        data: *const libc::c_void,
        data_byte_length: libc::c_ulong,
        cleanup_hndlr: Option<zbar_image_cleanup_handler_t>,
    );
    /* * built-in cleanup handler.
     * passes the image data buffer to free()
     */
    #[no_mangle]
    fn zbar_image_free_data(image: *mut zbar_image_t);
    /* * associate user specified data value with an image.
     * @since 0.5
     */
    #[no_mangle]
    fn zbar_image_set_userdata(image: *mut zbar_image_t, userdata: *mut libc::c_void);
    /* * return user specified data value associated with the image.
     * @since 0.5
     */
    #[no_mangle]
    fn zbar_image_get_userdata(image: *const zbar_image_t) -> *mut libc::c_void;
    #[no_mangle]
    fn zbarSymbolSet_FromSymbolSet(zsyms: *const zbar_symbol_set_t) -> *mut zbarSymbolSet;
    #[no_mangle]
    fn zbarSymbolIter_FromSymbolSet(syms: *mut zbarSymbolSet) -> *mut zbarSymbolIter;
    #[no_mangle]
    static mut zbarSymbolSet_Type: PyTypeObject;
    #[no_mangle]
    fn parse_dimensions(seq: *mut PyObject, dims: *mut libc::c_int, n: libc::c_int) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __intptr_t = libc::c_long;
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
pub type intptr_t = __intptr_t;
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
/* we used to include an E, backwards compatible alias */
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
pub type destructor = Option<unsafe extern fn(_: *mut PyObject) -> ()>;
pub type PyObject = _object;
pub type inquiry = Option<unsafe extern fn(_: *mut PyObject) -> libc::c_int>;
pub type freefunc = Option<unsafe extern fn(_: *mut libc::c_void) -> ()>;
pub type newfunc = Option<
    unsafe extern fn(_: *mut _typeobject, _: *mut PyObject, _: *mut PyObject) -> *mut PyObject,
>;
pub type allocfunc = Option<unsafe extern fn(_: *mut _typeobject, _: Py_ssize_t) -> *mut PyObject>;
pub type initproc =
    Option<unsafe extern fn(_: *mut PyObject, _: *mut PyObject, _: *mut PyObject) -> libc::c_int>;
pub type descrsetfunc =
    Option<unsafe extern fn(_: *mut PyObject, _: *mut PyObject, _: *mut PyObject) -> libc::c_int>;
pub type descrgetfunc =
    Option<unsafe extern fn(_: *mut PyObject, _: *mut PyObject, _: *mut PyObject) -> *mut PyObject>;
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
pub type setter = Option<
    unsafe extern fn(_: *mut PyObject, _: *mut PyObject, _: *mut libc::c_void) -> libc::c_int,
>;
pub type getter = Option<unsafe extern fn(_: *mut PyObject, _: *mut libc::c_void) -> *mut PyObject>;
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
pub type PyCFunction =
    Option<unsafe extern fn(_: *mut PyObject, _: *mut PyObject) -> *mut PyObject>;
pub type iternextfunc = Option<unsafe extern fn(_: *mut PyObject) -> *mut PyObject>;
pub type getiterfunc = Option<unsafe extern fn(_: *mut PyObject) -> *mut PyObject>;
pub type richcmpfunc =
    Option<unsafe extern fn(_: *mut PyObject, _: *mut PyObject, _: libc::c_int) -> *mut PyObject>;
pub type traverseproc =
    Option<unsafe extern fn(_: *mut PyObject, _: visitproc, _: *mut libc::c_void) -> libc::c_int>;
pub type visitproc =
    Option<unsafe extern fn(_: *mut PyObject, _: *mut libc::c_void) -> libc::c_int>;
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
pub type releasebufferproc = Option<unsafe extern fn(_: *mut PyObject, _: *mut Py_buffer) -> ()>;
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
pub type getbufferproc =
    Option<unsafe extern fn(_: *mut PyObject, _: *mut Py_buffer, _: libc::c_int) -> libc::c_int>;
pub type charbufferproc = Option<
    unsafe extern fn(_: *mut PyObject, _: Py_ssize_t, _: *mut *mut libc::c_char) -> Py_ssize_t,
>;
pub type segcountproc =
    Option<unsafe extern fn(_: *mut PyObject, _: *mut Py_ssize_t) -> Py_ssize_t>;
pub type writebufferproc = Option<
    unsafe extern fn(_: *mut PyObject, _: Py_ssize_t, _: *mut *mut libc::c_void) -> Py_ssize_t,
>;
pub type readbufferproc = Option<
    unsafe extern fn(_: *mut PyObject, _: Py_ssize_t, _: *mut *mut libc::c_void) -> Py_ssize_t,
>;
pub type setattrofunc =
    Option<unsafe extern fn(_: *mut PyObject, _: *mut PyObject, _: *mut PyObject) -> libc::c_int>;
pub type getattrofunc =
    Option<unsafe extern fn(_: *mut PyObject, _: *mut PyObject) -> *mut PyObject>;
pub type reprfunc = Option<unsafe extern fn(_: *mut PyObject) -> *mut PyObject>;
pub type ternaryfunc =
    Option<unsafe extern fn(_: *mut PyObject, _: *mut PyObject, _: *mut PyObject) -> *mut PyObject>;
pub type hashfunc = Option<unsafe extern fn(_: *mut PyObject) -> libc::c_long>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyMappingMethods {
    pub mp_length: lenfunc,
    pub mp_subscript: binaryfunc,
    pub mp_ass_subscript: objobjargproc,
}
pub type objobjargproc =
    Option<unsafe extern fn(_: *mut PyObject, _: *mut PyObject, _: *mut PyObject) -> libc::c_int>;
pub type binaryfunc = Option<unsafe extern fn(_: *mut PyObject, _: *mut PyObject) -> *mut PyObject>;
pub type lenfunc = Option<unsafe extern fn(_: *mut PyObject) -> Py_ssize_t>;
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
pub type ssizeargfunc = Option<unsafe extern fn(_: *mut PyObject, _: Py_ssize_t) -> *mut PyObject>;
pub type objobjproc = Option<unsafe extern fn(_: *mut PyObject, _: *mut PyObject) -> libc::c_int>;
pub type ssizessizeobjargproc = Option<
    unsafe extern fn(
        _: *mut PyObject,
        _: Py_ssize_t,
        _: Py_ssize_t,
        _: *mut PyObject,
    ) -> libc::c_int,
>;
pub type ssizeobjargproc =
    Option<unsafe extern fn(_: *mut PyObject, _: Py_ssize_t, _: *mut PyObject) -> libc::c_int>;
pub type ssizessizeargfunc =
    Option<unsafe extern fn(_: *mut PyObject, _: Py_ssize_t, _: Py_ssize_t) -> *mut PyObject>;
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
pub type unaryfunc = Option<unsafe extern fn(_: *mut PyObject) -> *mut PyObject>;
pub type coercion =
    Option<unsafe extern fn(_: *mut *mut PyObject, _: *mut *mut PyObject) -> libc::c_int>;
pub type cmpfunc = Option<unsafe extern fn(_: *mut PyObject, _: *mut PyObject) -> libc::c_int>;
pub type setattrfunc = Option<
    unsafe extern fn(_: *mut PyObject, _: *mut libc::c_char, _: *mut PyObject) -> libc::c_int,
>;
pub type getattrfunc =
    Option<unsafe extern fn(_: *mut PyObject, _: *mut libc::c_char) -> *mut PyObject>;
pub type printfunc =
    Option<unsafe extern fn(_: *mut PyObject, _: *mut FILE, _: libc::c_int) -> libc::c_int>;
pub type PyTypeObject = _typeobject;
pub type zbar_symbol_t = zbar_symbol_s;
pub type zbar_symbol_set_t = zbar_symbol_set_s;
/* * opaque image object. */
pub type zbar_image_t = zbar_image_s;
/* * cleanup handler callback function.
 * called to free sample data when an image is destroyed.
 */
pub type zbar_image_cleanup_handler_t = unsafe extern fn(_: *mut zbar_image_t) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zbarImage {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub zimg: *mut zbar_image_t,
    pub data: *mut PyObject,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zbarSymbolIter {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub zsym: *const zbar_symbol_t,
    pub syms: *mut zbarSymbolSet,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zbarSymbolSet {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub zsyms: *const zbar_symbol_set_t,
}
/* * consistently compute fourcc values across architectures
 * (adapted from v4l2 specification)
 * @since 0.11
 */
/* * parse a fourcc string into its encoded integer value.
 * @since 0.11
 */
#[inline]
unsafe extern fn zbar_fourcc_parse(mut format: *const libc::c_char) -> libc::c_ulong {
    let mut fourcc: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    if !format.is_null() {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int && *format.offset(i as isize) as libc::c_int != 0 {
            fourcc |= (*format.offset(i as isize) as libc::c_ulong) << i * 8 as libc::c_int;
            i += 1
        }
    }
    return fourcc;
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
 *------------------------------------------------------------------------ */
static mut image_doc: [libc::c_char; 89] = [
    105, 109, 97, 103, 101, 32, 111, 98, 106, 101, 99, 116, 46, 10, 10, 115, 116, 111, 114, 101,
    115, 32, 105, 109, 97, 103, 101, 32, 100, 97, 116, 97, 32, 115, 97, 109, 112, 108, 101, 115,
    32, 97, 108, 111, 110, 103, 32, 119, 105, 116, 104, 32, 97, 115, 115, 111, 99, 105, 97, 116,
    101, 100, 32, 102, 111, 114, 109, 97, 116, 32, 97, 110, 100, 32, 115, 105, 122, 101, 32, 109,
    101, 116, 97, 100, 97, 116, 97, 46, 0,
];
unsafe extern fn image_new(
    mut type_0: *mut PyTypeObject,
    mut args: *mut PyObject,
    mut kwds: *mut PyObject,
) -> *mut zbarImage {
    let mut self_0: *mut zbarImage = (*type_0).tp_alloc.expect("non-null function pointer")(
        type_0,
        0 as libc::c_int as Py_ssize_t,
    ) as *mut zbarImage;
    if self_0.is_null() {
        return 0 as *mut zbarImage;
    }
    (*self_0).zimg = zbar_image_create();
    if (*self_0).zimg.is_null() {
        let ref mut fresh0 = (*(self_0 as *mut PyObject)).ob_refcnt;
        *fresh0 -= 1;
        if !(*fresh0 != 0 as libc::c_int as libc::c_long) {
            Some(
                (*(*(self_0 as *mut PyObject)).ob_type)
                    .tp_dealloc
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(self_0 as *mut PyObject);
        }
        return 0 as *mut zbarImage;
    }
    zbar_image_set_userdata((*self_0).zimg, self_0 as *mut libc::c_void);
    return self_0;
}
unsafe extern fn image_traverse(
    mut self_0: *mut zbarImage,
    mut visit: visitproc,
    mut arg: *mut libc::c_void,
) -> libc::c_int {
    if !(*self_0).data.is_null() {
        let mut vret: libc::c_int = visit.expect("non-null function pointer")((*self_0).data, arg);
        if vret != 0 {
            return vret;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern fn image_clear(mut self_0: *mut zbarImage) -> libc::c_int {
    let mut zimg: *mut zbar_image_t = (*self_0).zimg;
    (*self_0).zimg = 0 as *mut zbar_image_t;
    if !zimg.is_null() {
        if !(*self_0).data.is_null() {
            /* attach data directly to zbar image */
            zbar_image_set_userdata(zimg, (*self_0).data as *mut libc::c_void); /* FIXME internal error */
            (*self_0).data = 0 as *mut PyObject
        } else {
            zbar_image_set_userdata(zimg, 0 as *mut libc::c_void);
        }
        zbar_image_destroy(zimg);
    }
    return 0 as libc::c_int;
}
unsafe extern fn image_dealloc(mut self_0: *mut zbarImage) {
    image_clear(self_0);
    (*(*(self_0 as *mut PyObject)).ob_type).tp_free.expect("non-null function pointer")(
        self_0 as *mut PyObject as *mut libc::c_void,
    );
}
unsafe extern fn image_get_symbols(
    mut self_0: *mut zbarImage,
    mut closure: *mut libc::c_void,
) -> *mut zbarSymbolSet {
    let mut zsyms: *const zbar_symbol_set_t = zbar_image_get_symbols((*self_0).zimg);
    return zbarSymbolSet_FromSymbolSet(zsyms);
}
unsafe extern fn image_set_symbols(
    mut self_0: *mut zbarImage,
    mut value: *mut PyObject,
    mut closure: *mut libc::c_void,
) -> libc::c_int {
    let mut zsyms: *const zbar_symbol_set_t = 0 as *const zbar_symbol_set_t;
    if value.is_null() || value == &mut _Py_NoneStruct as *mut PyObject {
        zsyms = 0 as *const zbar_symbol_set_t
    } else if (*value).ob_type == &mut zbarSymbolSet_Type as *mut PyTypeObject
        || PyType_IsSubtype((*value).ob_type, &mut zbarSymbolSet_Type) != 0
    {
        zsyms = (*(value as *mut zbarSymbolSet)).zsyms
    } else {
        PyErr_Format(
            PyExc_TypeError,
            b"must set image symbols to a zbar.SymbolSet, not \'%.50s\'\x00" as *const u8
                as *const libc::c_char,
            (*(*value).ob_type).tp_name,
        );
        return -(1 as libc::c_int);
    }
    zbar_image_set_symbols((*self_0).zimg, zsyms);
    return 0 as libc::c_int;
}
unsafe extern fn image_iter(mut self_0: *mut zbarImage) -> *mut zbarSymbolIter {
    let mut syms: *mut zbarSymbolSet = image_get_symbols(self_0, 0 as *mut libc::c_void);
    if syms.is_null() {
        return 0 as *mut zbarSymbolIter;
    }
    return zbarSymbolIter_FromSymbolSet(syms);
}
unsafe extern fn image_get_format(
    mut self_0: *mut zbarImage,
    mut closure: *mut libc::c_void,
) -> *mut PyObject {
    let mut format: libc::c_ulong = zbar_image_get_format((*self_0).zimg);
    return PyString_FromStringAndSize(
        &mut format as *mut libc::c_ulong as *mut libc::c_char,
        4 as libc::c_int as Py_ssize_t,
    );
}
unsafe extern fn image_set_format(
    mut self_0: *mut zbarImage,
    mut value: *mut PyObject,
    mut closure: *mut libc::c_void,
) -> libc::c_int {
    if value.is_null() {
        PyErr_SetString(
            PyExc_TypeError,
            b"cannot delete format attribute\x00" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    let mut format: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: Py_ssize_t = 0;
    if PyString_AsStringAndSize(value, &mut format, &mut len) != 0
        || format.is_null()
        || len != 4 as libc::c_int as libc::c_long
    {
        if format.is_null() {
            format = b"(nil)\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
        }
        PyErr_Format(
            PyExc_ValueError,
            b"format \'%.50s\' is not a valid four character code\x00" as *const u8
                as *const libc::c_char,
            format,
        );
        return -(1 as libc::c_int);
    }
    zbar_image_set_format((*self_0).zimg, zbar_fourcc_parse(format));
    return 0 as libc::c_int;
}
unsafe extern fn image_get_size(
    mut self_0: *mut zbarImage,
    mut closure: *mut libc::c_void,
) -> *mut PyObject {
    let mut w: libc::c_uint = 0;
    let mut h: libc::c_uint = 0;
    zbar_image_get_size((*self_0).zimg, &mut w, &mut h);
    return PyTuple_Pack(
        2 as libc::c_int as Py_ssize_t,
        PyInt_FromLong(w as libc::c_long),
        PyInt_FromLong(h as libc::c_long),
    );
}
unsafe extern fn image_set_size(
    mut self_0: *mut zbarImage,
    mut value: *mut PyObject,
    mut closure: *mut libc::c_void,
) -> libc::c_int {
    if value.is_null() {
        PyErr_SetString(
            PyExc_TypeError,
            b"cannot delete size attribute\x00" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    let mut dims: [libc::c_int; 2] = [0; 2];
    if parse_dimensions(value, dims.as_mut_ptr(), 2 as libc::c_int) != 0
        || dims[0 as libc::c_int as usize] < 0 as libc::c_int
        || dims[1 as libc::c_int as usize] < 0 as libc::c_int
    {
        PyErr_SetString(
            PyExc_ValueError,
            b"size must be a sequence of two positive ints\x00" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    zbar_image_set_size(
        (*self_0).zimg,
        dims[0 as libc::c_int as usize] as libc::c_uint,
        dims[1 as libc::c_int as usize] as libc::c_uint,
    );
    return 0 as libc::c_int;
}
unsafe extern fn image_get_crop(
    mut self_0: *mut zbarImage,
    mut closure: *mut libc::c_void,
) -> *mut PyObject {
    let mut x: libc::c_uint = 0;
    let mut y: libc::c_uint = 0;
    let mut w: libc::c_uint = 0;
    let mut h: libc::c_uint = 0;
    zbar_image_get_crop((*self_0).zimg, &mut x, &mut y, &mut w, &mut h);
    return PyTuple_Pack(
        4 as libc::c_int as Py_ssize_t,
        PyInt_FromLong(x as libc::c_long),
        PyInt_FromLong(y as libc::c_long),
        PyInt_FromLong(w as libc::c_long),
        PyInt_FromLong(h as libc::c_long),
    );
}
unsafe extern fn image_set_crop(
    mut self_0: *mut zbarImage,
    mut value: *mut PyObject,
    mut closure: *mut libc::c_void,
) -> libc::c_int {
    let mut w: libc::c_uint = 0;
    let mut h: libc::c_uint = 0;
    zbar_image_get_size((*self_0).zimg, &mut w, &mut h);
    if value.is_null() {
        zbar_image_set_crop(
            (*self_0).zimg,
            0 as libc::c_int as libc::c_uint,
            0 as libc::c_int as libc::c_uint,
            w,
            h,
        );
        return 0 as libc::c_int;
    }
    let mut dims: [libc::c_int; 4] = [0; 4];
    if parse_dimensions(value, dims.as_mut_ptr(), 4 as libc::c_int) != 0
        || dims[2 as libc::c_int as usize] < 0 as libc::c_int
        || dims[3 as libc::c_int as usize] < 0 as libc::c_int
    {
        PyErr_SetString(
            PyExc_ValueError,
            b"crop must be a sequence of four positive ints\x00" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if dims[0 as libc::c_int as usize] < 0 as libc::c_int {
        dims[2 as libc::c_int as usize] += dims[0 as libc::c_int as usize];
        dims[0 as libc::c_int as usize] = 0 as libc::c_int
    }
    if dims[1 as libc::c_int as usize] < 0 as libc::c_int {
        dims[3 as libc::c_int as usize] += dims[1 as libc::c_int as usize];
        dims[1 as libc::c_int as usize] = 0 as libc::c_int
    }
    zbar_image_set_crop(
        (*self_0).zimg,
        dims[0 as libc::c_int as usize] as libc::c_uint,
        dims[1 as libc::c_int as usize] as libc::c_uint,
        dims[2 as libc::c_int as usize] as libc::c_uint,
        dims[3 as libc::c_int as usize] as libc::c_uint,
    );
    return 0 as libc::c_int;
}
unsafe extern fn image_get_int(
    mut self_0: *mut zbarImage,
    mut closure: *mut libc::c_void,
) -> *mut PyObject {
    let mut val: libc::c_uint = -(1 as libc::c_int) as libc::c_uint;
    match closure as intptr_t {
        0 => val = zbar_image_get_width((*self_0).zimg),
        1 => val = zbar_image_get_height((*self_0).zimg),
        2 => val = zbar_image_get_sequence((*self_0).zimg),
        _ => {}
    }
    return PyInt_FromLong(val as libc::c_long);
}
unsafe extern fn image_set_int(
    mut self_0: *mut zbarImage,
    mut value: *mut PyObject,
    mut closure: *mut libc::c_void,
) -> libc::c_int {
    let mut tmp: libc::c_uint = 0;
    let mut val: libc::c_uint = PyInt_AsSsize_t(value) as libc::c_uint;
    if val == -(1 as libc::c_int) as libc::c_uint && !PyErr_Occurred().is_null() {
        PyErr_SetString(
            PyExc_TypeError,
            b"expecting an integer\x00" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    match closure as intptr_t {
        0 => {
            tmp = zbar_image_get_height((*self_0).zimg);
            zbar_image_set_size((*self_0).zimg, val, tmp);
        }
        1 => {
            tmp = zbar_image_get_width((*self_0).zimg);
            zbar_image_set_size((*self_0).zimg, tmp, val);
        }
        2 => {
            zbar_image_set_sequence((*self_0).zimg, val);
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
unsafe extern fn image_get_data(
    mut self_0: *mut zbarImage,
    mut closure: *mut libc::c_void,
) -> *mut PyObject {
    if !(*self_0).data.is_null() {
        (*(*self_0).data).ob_refcnt += 1;
        return (*self_0).data;
    }
    let mut data: *const libc::c_char = zbar_image_get_data((*self_0).zimg) as *const libc::c_char;
    let mut datalen: libc::c_ulong = zbar_image_get_data_length((*self_0).zimg);
    if data.is_null() || datalen == 0 {
        let ref mut fresh1 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh1 += 1;
        return &mut _Py_NoneStruct;
    }
    (*self_0).data = PyBuffer_FromMemory(data as *mut libc::c_void, datalen as Py_ssize_t);
    (*(*self_0).data).ob_refcnt += 1;
    return (*self_0).data;
}
#[no_mangle]
pub unsafe extern fn image_cleanup(mut zimg: *mut zbar_image_t) {
    let mut data: *mut PyObject = zbar_image_get_userdata(zimg) as *mut PyObject;
    zbar_image_set_userdata(zimg, 0 as *mut libc::c_void);
    if data.is_null() {
        return;
    }
    if (*data).ob_type == &mut zbarImage_Type as *mut PyTypeObject
        || PyType_IsSubtype((*data).ob_type, &mut zbarImage_Type) != 0
    {
        let mut self_0: *mut zbarImage = data as *mut zbarImage;
        if !(*self_0).data.is_null() {
            let mut _py_tmp: *mut PyObject = (*self_0).data;
            (*self_0).data = 0 as *mut PyObject;
            (*_py_tmp).ob_refcnt -= 1;
            if !((*_py_tmp).ob_refcnt != 0 as libc::c_int as libc::c_long) {
                Some((*(*_py_tmp).ob_type).tp_dealloc.expect("non-null function pointer"))
                    .expect("non-null function pointer")(_py_tmp);
            }
        }
    } else {
        (*data).ob_refcnt -= 1;
        if !((*data).ob_refcnt != 0 as libc::c_int as libc::c_long) {
            Some((*(*data).ob_type).tp_dealloc.expect("non-null function pointer"))
                .expect("non-null function pointer")(data);
        }
    };
}
unsafe extern fn image_set_data(
    mut self_0: *mut zbarImage,
    mut value: *mut PyObject,
    mut closure: *mut libc::c_void,
) -> libc::c_int {
    if value.is_null() {
        zbar_image_free_data((*self_0).zimg);
        return 0 as libc::c_int;
    }
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut datalen: Py_ssize_t = 0;
    if PyString_AsStringAndSize(value, &mut data, &mut datalen) != 0 {
        return -(1 as libc::c_int);
    }
    (*value).ob_refcnt += 1;
    zbar_image_set_data(
        (*self_0).zimg,
        data as *const libc::c_void,
        datalen as libc::c_ulong,
        Some(image_cleanup as unsafe extern fn(_: *mut zbar_image_t) -> ()),
    );
    (*self_0).data = value;
    zbar_image_set_userdata((*self_0).zimg, self_0 as *mut libc::c_void);
    return 0 as libc::c_int;
}
static mut image_getset: [PyGetSetDef; 9] = unsafe {
    [
        {
            let mut init = PyGetSetDef {
                name: b"format\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                get: ::std::mem::transmute::<
                    Option<
                        unsafe extern fn(_: *mut zbarImage, _: *mut libc::c_void) -> *mut PyObject,
                    >,
                    getter,
                >(Some(
                    image_get_format
                        as unsafe extern fn(
                            _: *mut zbarImage,
                            _: *mut libc::c_void,
                        ) -> *mut PyObject,
                )),
                set: ::std::mem::transmute::<
                    Option<
                        unsafe extern fn(
                            _: *mut zbarImage,
                            _: *mut PyObject,
                            _: *mut libc::c_void,
                        ) -> libc::c_int,
                    >,
                    setter,
                >(Some(
                    image_set_format
                        as unsafe extern fn(
                            _: *mut zbarImage,
                            _: *mut PyObject,
                            _: *mut libc::c_void,
                        ) -> libc::c_int,
                )),
                doc: 0 as *const libc::c_char as *mut libc::c_char,
                closure: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = PyGetSetDef {
                name: b"size\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                get: ::std::mem::transmute::<
                    Option<
                        unsafe extern fn(_: *mut zbarImage, _: *mut libc::c_void) -> *mut PyObject,
                    >,
                    getter,
                >(Some(
                    image_get_size
                        as unsafe extern fn(
                            _: *mut zbarImage,
                            _: *mut libc::c_void,
                        ) -> *mut PyObject,
                )),
                set: ::std::mem::transmute::<
                    Option<
                        unsafe extern fn(
                            _: *mut zbarImage,
                            _: *mut PyObject,
                            _: *mut libc::c_void,
                        ) -> libc::c_int,
                    >,
                    setter,
                >(Some(
                    image_set_size
                        as unsafe extern fn(
                            _: *mut zbarImage,
                            _: *mut PyObject,
                            _: *mut libc::c_void,
                        ) -> libc::c_int,
                )),
                doc: 0 as *const libc::c_char as *mut libc::c_char,
                closure: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = PyGetSetDef {
                name: b"crop\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                get: ::std::mem::transmute::<
                    Option<
                        unsafe extern fn(_: *mut zbarImage, _: *mut libc::c_void) -> *mut PyObject,
                    >,
                    getter,
                >(Some(
                    image_get_crop
                        as unsafe extern fn(
                            _: *mut zbarImage,
                            _: *mut libc::c_void,
                        ) -> *mut PyObject,
                )),
                set: ::std::mem::transmute::<
                    Option<
                        unsafe extern fn(
                            _: *mut zbarImage,
                            _: *mut PyObject,
                            _: *mut libc::c_void,
                        ) -> libc::c_int,
                    >,
                    setter,
                >(Some(
                    image_set_crop
                        as unsafe extern fn(
                            _: *mut zbarImage,
                            _: *mut PyObject,
                            _: *mut libc::c_void,
                        ) -> libc::c_int,
                )),
                doc: 0 as *const libc::c_char as *mut libc::c_char,
                closure: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = PyGetSetDef {
                name: b"width\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                get: ::std::mem::transmute::<
                    Option<
                        unsafe extern fn(_: *mut zbarImage, _: *mut libc::c_void) -> *mut PyObject,
                    >,
                    getter,
                >(Some(
                    image_get_int
                        as unsafe extern fn(
                            _: *mut zbarImage,
                            _: *mut libc::c_void,
                        ) -> *mut PyObject,
                )),
                set: ::std::mem::transmute::<
                    Option<
                        unsafe extern fn(
                            _: *mut zbarImage,
                            _: *mut PyObject,
                            _: *mut libc::c_void,
                        ) -> libc::c_int,
                    >,
                    setter,
                >(Some(
                    image_set_int
                        as unsafe extern fn(
                            _: *mut zbarImage,
                            _: *mut PyObject,
                            _: *mut libc::c_void,
                        ) -> libc::c_int,
                )),
                doc: 0 as *const libc::c_char as *mut libc::c_char,
                closure: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = PyGetSetDef {
                name: b"height\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                get: ::std::mem::transmute::<
                    Option<
                        unsafe extern fn(_: *mut zbarImage, _: *mut libc::c_void) -> *mut PyObject,
                    >,
                    getter,
                >(Some(
                    image_get_int
                        as unsafe extern fn(
                            _: *mut zbarImage,
                            _: *mut libc::c_void,
                        ) -> *mut PyObject,
                )),
                set: ::std::mem::transmute::<
                    Option<
                        unsafe extern fn(
                            _: *mut zbarImage,
                            _: *mut PyObject,
                            _: *mut libc::c_void,
                        ) -> libc::c_int,
                    >,
                    setter,
                >(Some(
                    image_set_int
                        as unsafe extern fn(
                            _: *mut zbarImage,
                            _: *mut PyObject,
                            _: *mut libc::c_void,
                        ) -> libc::c_int,
                )),
                doc: 0 as *const libc::c_char as *mut libc::c_char,
                closure: 1 as libc::c_int as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = PyGetSetDef {
                name: b"sequence\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                get: ::std::mem::transmute::<
                    Option<
                        unsafe extern fn(_: *mut zbarImage, _: *mut libc::c_void) -> *mut PyObject,
                    >,
                    getter,
                >(Some(
                    image_get_int
                        as unsafe extern fn(
                            _: *mut zbarImage,
                            _: *mut libc::c_void,
                        ) -> *mut PyObject,
                )),
                set: ::std::mem::transmute::<
                    Option<
                        unsafe extern fn(
                            _: *mut zbarImage,
                            _: *mut PyObject,
                            _: *mut libc::c_void,
                        ) -> libc::c_int,
                    >,
                    setter,
                >(Some(
                    image_set_int
                        as unsafe extern fn(
                            _: *mut zbarImage,
                            _: *mut PyObject,
                            _: *mut libc::c_void,
                        ) -> libc::c_int,
                )),
                doc: 0 as *const libc::c_char as *mut libc::c_char,
                closure: 2 as libc::c_int as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = PyGetSetDef {
                name: b"data\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                get: ::std::mem::transmute::<
                    Option<
                        unsafe extern fn(_: *mut zbarImage, _: *mut libc::c_void) -> *mut PyObject,
                    >,
                    getter,
                >(Some(
                    image_get_data
                        as unsafe extern fn(
                            _: *mut zbarImage,
                            _: *mut libc::c_void,
                        ) -> *mut PyObject,
                )),
                set: ::std::mem::transmute::<
                    Option<
                        unsafe extern fn(
                            _: *mut zbarImage,
                            _: *mut PyObject,
                            _: *mut libc::c_void,
                        ) -> libc::c_int,
                    >,
                    setter,
                >(Some(
                    image_set_data
                        as unsafe extern fn(
                            _: *mut zbarImage,
                            _: *mut PyObject,
                            _: *mut libc::c_void,
                        ) -> libc::c_int,
                )),
                doc: 0 as *const libc::c_char as *mut libc::c_char,
                closure: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = PyGetSetDef {
                name: b"symbols\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                get: ::std::mem::transmute::<
                    Option<
                        unsafe extern fn(
                            _: *mut zbarImage,
                            _: *mut libc::c_void,
                        ) -> *mut zbarSymbolSet,
                    >,
                    getter,
                >(Some(
                    image_get_symbols
                        as unsafe extern fn(
                            _: *mut zbarImage,
                            _: *mut libc::c_void,
                        ) -> *mut zbarSymbolSet,
                )),
                set: ::std::mem::transmute::<
                    Option<
                        unsafe extern fn(
                            _: *mut zbarImage,
                            _: *mut PyObject,
                            _: *mut libc::c_void,
                        ) -> libc::c_int,
                    >,
                    setter,
                >(Some(
                    image_set_symbols
                        as unsafe extern fn(
                            _: *mut zbarImage,
                            _: *mut PyObject,
                            _: *mut libc::c_void,
                        ) -> libc::c_int,
                )),
                doc: 0 as *const libc::c_char as *mut libc::c_char,
                closure: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = PyGetSetDef {
                name: 0 as *const libc::c_char as *mut libc::c_char,
                get: None,
                set: None,
                doc: 0 as *const libc::c_char as *mut libc::c_char,
                closure: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
    ]
};
unsafe extern fn image_init(
    mut self_0: *mut zbarImage,
    mut args: *mut PyObject,
    mut kwds: *mut PyObject,
) -> libc::c_int {
    let mut width: libc::c_int = -(1 as libc::c_int);
    let mut height: libc::c_int = -(1 as libc::c_int);
    let mut format: *mut PyObject = 0 as *mut PyObject;
    let mut data: *mut PyObject = 0 as *mut PyObject;
    static mut kwlist: [*mut libc::c_char; 5] = [
        b"width\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"height\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"format\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"data\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
    ];
    if PyArg_ParseTupleAndKeywords(
        args,
        kwds,
        b"|iiOO\x00" as *const u8 as *const libc::c_char,
        kwlist.as_mut_ptr(),
        &mut width as *mut libc::c_int,
        &mut height as *mut libc::c_int,
        &mut format as *mut *mut PyObject,
        &mut data as *mut *mut PyObject,
    ) == 0
    {
        return -(1 as libc::c_int);
    }
    if width > 0 as libc::c_int && height > 0 as libc::c_int {
        zbar_image_set_size((*self_0).zimg, width as libc::c_uint, height as libc::c_uint);
    }
    if !format.is_null() && image_set_format(self_0, format, 0 as *mut libc::c_void) != 0 {
        return -(1 as libc::c_int);
    }
    if !data.is_null() && image_set_data(self_0, data, 0 as *mut libc::c_void) != 0 {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern fn image_convert(
    mut self_0: *mut zbarImage,
    mut args: *mut PyObject,
    mut kwds: *mut PyObject,
) -> *mut zbarImage {
    let mut format: *const libc::c_char = 0 as *const libc::c_char;
    let mut width: libc::c_int = -(1 as libc::c_int);
    let mut height: libc::c_int = -(1 as libc::c_int);
    static mut kwlist: [*mut libc::c_char; 4] = [
        b"format\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"width\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"height\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
    ];
    if PyArg_ParseTupleAndKeywords(
        args,
        kwds,
        b"s|ii\x00" as *const u8 as *const libc::c_char,
        kwlist.as_mut_ptr(),
        &mut format as *mut *const libc::c_char,
        &mut width as *mut libc::c_int,
        &mut height as *mut libc::c_int,
    ) == 0
    {
        return 0 as *mut zbarImage;
    }
    if strlen(format) != 4 as libc::c_int as libc::c_ulong {
        PyErr_Format(
            PyExc_ValueError,
            b"format \'%.50s\' is not a valid four character code\x00" as *const u8
                as *const libc::c_char,
            format,
        );
        return 0 as *mut zbarImage;
    }
    let mut fourcc: libc::c_ulong = zbar_fourcc_parse(format);
    let mut img: *mut zbarImage = _PyObject_GC_New(&mut zbarImage_Type) as *mut zbarImage;
    if img.is_null() {
        return 0 as *mut zbarImage;
    }
    (*img).data = 0 as *mut PyObject;
    if width > 0 as libc::c_int && height > 0 as libc::c_int {
        (*img).zimg = zbar_image_convert_resize(
            (*self_0).zimg,
            fourcc,
            width as libc::c_uint,
            height as libc::c_uint,
        )
    } else {
        (*img).zimg = zbar_image_convert((*self_0).zimg, fourcc)
    }
    if (*img).zimg.is_null() {
        /* FIXME propagate exception */
        let ref mut fresh2 = (*(img as *mut PyObject)).ob_refcnt;
        *fresh2 -= 1;
        if !(*fresh2 != 0 as libc::c_int as libc::c_long) {
            Some(
                (*(*(img as *mut PyObject)).ob_type).tp_dealloc.expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(img as *mut PyObject);
        }
        return 0 as *mut zbarImage;
    }
    zbar_image_set_userdata((*img).zimg, img as *mut libc::c_void);
    return img;
}
static mut image_methods: [PyMethodDef; 2] = unsafe {
    [
        {
            let mut init = PyMethodDef {
                ml_name: b"convert\x00" as *const u8 as *const libc::c_char,
                ml_meth: ::std::mem::transmute::<
                    Option<
                        unsafe extern fn(
                            _: *mut zbarImage,
                            _: *mut PyObject,
                            _: *mut PyObject,
                        ) -> *mut zbarImage,
                    >,
                    PyCFunction,
                >(Some(
                    image_convert
                        as unsafe extern fn(
                            _: *mut zbarImage,
                            _: *mut PyObject,
                            _: *mut PyObject,
                        ) -> *mut zbarImage,
                )),
                ml_flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: 0 as *const libc::c_char,
                ml_meth: None,
                ml_flags: 0,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut zbarImage_Type: PyTypeObject = unsafe {
    {
        let mut init = _typeobject {
            ob_refcnt: 1 as libc::c_int as Py_ssize_t,
            ob_type: 0 as *const _typeobject as *mut _typeobject,
            ob_size: 0,
            tp_name: b"zbar.Image\x00" as *const u8 as *const libc::c_char,
            tp_basicsize: ::std::mem::size_of::<zbarImage>() as libc::c_ulong as Py_ssize_t,
            tp_itemsize: 0,
            tp_dealloc: ::std::mem::transmute::<
                Option<unsafe extern fn(_: *mut zbarImage) -> ()>,
                destructor,
            >(Some(
                image_dealloc as unsafe extern fn(_: *mut zbarImage) -> (),
            )),
            tp_print: None,
            tp_getattr: None,
            tp_setattr: None,
            tp_compare: None,
            tp_repr: None,
            tp_as_number: 0 as *const PyNumberMethods as *mut PyNumberMethods,
            tp_as_sequence: 0 as *const PySequenceMethods as *mut PySequenceMethods,
            tp_as_mapping: 0 as *const PyMappingMethods as *mut PyMappingMethods,
            tp_hash: None,
            tp_call: None,
            tp_str: None,
            tp_getattro: None,
            tp_setattro: None,
            tp_as_buffer: 0 as *const PyBufferProcs as *mut PyBufferProcs,
            tp_flags: (1 as libc::c_long) << 0 as libc::c_int
                | (1 as libc::c_long) << 1 as libc::c_int
                | (1 as libc::c_long) << 3 as libc::c_int
                | (1 as libc::c_long) << 5 as libc::c_int
                | (1 as libc::c_long) << 6 as libc::c_int
                | (1 as libc::c_long) << 7 as libc::c_int
                | (1 as libc::c_long) << 8 as libc::c_int
                | 0 as libc::c_int as libc::c_long
                | (1 as libc::c_long) << 17 as libc::c_int
                | 0 as libc::c_int as libc::c_long
                | (1 as libc::c_long) << 10 as libc::c_int
                | (1 as libc::c_long) << 14 as libc::c_int,
            tp_doc: image_doc.as_ptr() as *mut _,
            tp_traverse: ::std::mem::transmute::<
                Option<
                    unsafe extern fn(
                        _: *mut zbarImage,
                        _: visitproc,
                        _: *mut libc::c_void,
                    ) -> libc::c_int,
                >,
                traverseproc,
            >(Some(
                image_traverse
                    as unsafe extern fn(
                        _: *mut zbarImage,
                        _: visitproc,
                        _: *mut libc::c_void,
                    ) -> libc::c_int,
            )),
            tp_clear: ::std::mem::transmute::<
                Option<unsafe extern fn(_: *mut zbarImage) -> libc::c_int>,
                inquiry,
            >(Some(
                image_clear as unsafe extern fn(_: *mut zbarImage) -> libc::c_int,
            )),
            tp_richcompare: None,
            tp_weaklistoffset: 0,
            tp_iter: ::std::mem::transmute::<
                Option<unsafe extern fn(_: *mut zbarImage) -> *mut zbarSymbolIter>,
                getiterfunc,
            >(Some(
                image_iter as unsafe extern fn(_: *mut zbarImage) -> *mut zbarSymbolIter,
            )),
            tp_iternext: None,
            tp_methods: image_methods.as_ptr() as *mut _,
            tp_members: 0 as *const PyMemberDef as *mut PyMemberDef,
            tp_getset: image_getset.as_ptr() as *mut _,
            tp_base: 0 as *const _typeobject as *mut _typeobject,
            tp_dict: 0 as *const PyObject as *mut PyObject,
            tp_descr_get: None,
            tp_descr_set: None,
            tp_dictoffset: 0,
            tp_init: ::std::mem::transmute::<
                Option<
                    unsafe extern fn(
                        _: *mut zbarImage,
                        _: *mut PyObject,
                        _: *mut PyObject,
                    ) -> libc::c_int,
                >,
                initproc,
            >(Some(
                image_init
                    as unsafe extern fn(
                        _: *mut zbarImage,
                        _: *mut PyObject,
                        _: *mut PyObject,
                    ) -> libc::c_int,
            )),
            tp_alloc: None,
            tp_new: ::std::mem::transmute::<
                Option<
                    unsafe extern fn(
                        _: *mut PyTypeObject,
                        _: *mut PyObject,
                        _: *mut PyObject,
                    ) -> *mut zbarImage,
                >,
                newfunc,
            >(Some(
                image_new
                    as unsafe extern fn(
                        _: *mut PyTypeObject,
                        _: *mut PyObject,
                        _: *mut PyObject,
                    ) -> *mut zbarImage,
            )),
            tp_free: None,
            tp_is_gc: None,
            tp_bases: 0 as *const PyObject as *mut PyObject,
            tp_mro: 0 as *const PyObject as *mut PyObject,
            tp_cache: 0 as *const PyObject as *mut PyObject,
            tp_subclasses: 0 as *const PyObject as *mut PyObject,
            tp_weaklist: 0 as *const PyObject as *mut PyObject,
            tp_del: None,
            tp_version_tag: 0,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern fn zbarImage_FromImage(mut zimg: *mut zbar_image_t) -> *mut zbarImage {
    let mut self_0: *mut zbarImage = _PyObject_GC_New(&mut zbarImage_Type) as *mut zbarImage;
    if self_0.is_null() {
        return 0 as *mut zbarImage;
    }
    zbar_image_ref(zimg, 1 as libc::c_int);
    zbar_image_set_userdata(zimg, self_0 as *mut libc::c_void);
    (*self_0).zimg = zimg;
    (*self_0).data = 0 as *mut PyObject;
    return self_0;
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
 *------------------------------------------------------------------------ */
/* integer value is super type */
/* associated string name */
/* zbarEnumItem content dictionaries */
#[no_mangle]
pub unsafe extern fn zbarImage_validate(mut img: *mut zbarImage) -> libc::c_int {
    if zbar_image_get_width((*img).zimg) == 0
        || zbar_image_get_height((*img).zimg) == 0
        || zbar_image_get_data((*img).zimg).is_null()
        || zbar_image_get_data_length((*img).zimg) == 0
    {
        PyErr_Format(
            PyExc_ValueError,
            b"image size and data must be defined\x00" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
