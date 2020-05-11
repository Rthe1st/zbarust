use ::libc;
extern {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type PyMemberDef;
    /* Generator object interface */
    pub type _frame;
    pub type zbar_symbol_set_s;
    /* @} */
    /* ------------------------------------------------------------ */
    /* * @name Image interface
     * stores image data samples along with associated format and size
     * metadata
     */
    /* @{ */
    pub type zbar_image_s;
    /* @} */
    /* ------------------------------------------------------------ */
    /* * @name Processor interface
     * @anchor c-processor
     * high-level self-contained image processor.
     * processes video and images for barcodes, optionally displaying
     * images to a library owned output window
     */
    /* @{ */
    pub type zbar_processor_s;
    /* access macro to the members which are floating "behind" the object */
    /* Generic type check */
    #[no_mangle]
    fn PyType_IsSubtype(_: *mut PyTypeObject, _: *mut PyTypeObject) -> libc::c_int;
    #[no_mangle]
    fn PyObject_IsTrue(_: *mut PyObject) -> libc::c_int;
    #[no_mangle]
    fn PyCallable_Check(_: *mut PyObject) -> libc::c_int;
    /*
    _Py_NoneStruct is an object of undefined type which can be used in contexts
    where NULL (nil) is not suitable (since NULL often means 'error').

    Don't forget to apply Py_INCREF() when returning this value!!!
    */
    #[no_mangle]
    static mut _Py_NoneStruct: PyObject;
    #[no_mangle]
    fn parse_dimensions(seq: *mut PyObject, dims: *mut libc::c_int, n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn object_to_bool(obj: *mut PyObject, val: *mut libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn PyInt_FromLong(_: libc::c_long) -> *mut PyObject;
    #[no_mangle]
    fn PyInt_AsLong(_: *mut PyObject) -> libc::c_long;
    /* Use these macros */
    /* Macros for returning Py_True or Py_False, respectively */
    /* Function to return a bool from a C long */
    #[no_mangle]
    fn PyBool_FromLong(_: libc::c_long) -> *mut PyObject;
    #[no_mangle]
    static mut PyFloat_Type: PyTypeObject;
    #[no_mangle]
    fn PyTuple_New(size: Py_ssize_t) -> *mut PyObject;
    #[no_mangle]
    fn PyErr_SetString(_: *mut PyObject, _: *const libc::c_char);
    #[no_mangle]
    fn PyErr_Occurred() -> *mut PyObject;
    #[no_mangle]
    static mut PyExc_TypeError: *mut PyObject;
    #[no_mangle]
    static mut PyExc_ValueError: *mut PyObject;
    #[no_mangle]
    fn PyErr_NoMemory() -> *mut PyObject;
    /* MS_WINDOWS */
    #[no_mangle]
    fn PyErr_Format(_: *mut PyObject, _: *const libc::c_char, _: ...) -> *mut PyObject;
    /* Ensure that the current thread is ready to call the Python
       C API, regardless of the current state of Python, or of its
       thread lock.  This may be called as many times as desired
       by a thread so long as each call is matched with a call to
       PyGILState_Release().  In general, other thread-state APIs may
       be used between _Ensure() and _Release() calls, so long as the
       thread-state is restored to its previous state before the Release().
       For example, normal use of the Py_BEGIN_ALLOW_THREADS/
       Py_END_ALLOW_THREADS macros are acceptable.

       The return value is an opaque "handle" to the thread state when
       PyGILState_Ensure() was called, and must be passed to
       PyGILState_Release() to ensure Python is left in the same state. Even
       though recursive calls are allowed, these handles can *not* be shared -
       each unique call to PyGILState_Ensure must save the handle for its
       call to PyGILState_Release.

       When the function returns, the current thread will hold the GIL.

       Failure is a fatal error.
    */
    #[no_mangle]
    fn PyGILState_Ensure() -> PyGILState_STATE;
    /* Release any resources previously acquired.  After this call, Python's
       state will be the same as it was prior to the corresponding
       PyGILState_Ensure() call (but generally this state will be unknown to
       the caller, hence the use of the GILState API.)

       Every call to PyGILState_Ensure must be matched by a call to
       PyGILState_Release on the same thread.
    */
    #[no_mangle]
    fn PyGILState_Release(_: PyGILState_STATE);
    #[no_mangle]
    fn PyArg_ParseTupleAndKeywords(
        _: *mut PyObject,
        _: *mut PyObject,
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: ...
    ) -> libc::c_int;
    #[no_mangle]
    fn PyErr_Print();
    /* Interface for threads.

       A module that plans to do a blocking system call (or something else
       that lasts a long time and doesn't touch Python data) can allow other
       threads to run as follows:

        ...preparations here...
        Py_BEGIN_ALLOW_THREADS
        ...blocking system call here...
        Py_END_ALLOW_THREADS
        ...interpret result here...

       The Py_BEGIN_ALLOW_THREADS/Py_END_ALLOW_THREADS pair expands to a
       {}-surrounded block.
       To leave the block in the middle (e.g., with return), you must insert
       a line containing Py_BLOCK_THREADS before the return, e.g.

        if (...premature_exit...) {
            Py_BLOCK_THREADS
            PyErr_SetFromErrno(PyExc_IOError);
            return NULL;
        }

       An alternative is:

        Py_BLOCK_THREADS
        if (...premature_exit...) {
            PyErr_SetFromErrno(PyExc_IOError);
            return NULL;
        }
        Py_UNBLOCK_THREADS

       For convenience, that the value of 'errno' is restored across
       Py_END_ALLOW_THREADS and Py_BLOCK_THREADS.

       WARNING: NEVER NEST CALLS TO Py_BEGIN_ALLOW_THREADS AND
       Py_END_ALLOW_THREADS!!!

       The function PyEval_InitThreads() should be called only from
       initthread() in "threadmodule.c".

       Note that not yet all candidates have been converted to use this
       mechanism!
    */
    #[no_mangle]
    fn PyEval_SaveThread() -> *mut PyThreadState;
    #[no_mangle]
    fn PyEval_RestoreThread(_: *mut PyThreadState);
    #[no_mangle]
    fn PyEval_InitThreads();
    #[no_mangle]
    fn PySys_WriteStderr(format: *const libc::c_char, _: ...);
    /*
    Compare the values of o1 and o2 using a routine provided by
    o1, if one exists, otherwise with a routine provided by o2.
    The result of the comparison is returned in result.  Returns
    -1 on failure.  This is the equivalent of the Python
    statement: result=cmp(o1,o2).

      */
    /* Implemented elsewhere:

    int PyObject_Compare(PyObject *o1, PyObject *o2);

    Compare the values of o1 and o2 using a routine provided by
    o1, if one exists, otherwise with a routine provided by o2.
    Returns the result of the comparison on success.  On error,
    the value returned is undefined. This is equivalent to the
    Python expression: cmp(o1,o2).

      */
    /* Implemented elsewhere:

    PyObject *PyObject_Repr(PyObject *o);

    Compute the string representation of object, o.  Returns the
    string representation on success, NULL on failure.  This is
    the equivalent of the Python expression: repr(o).

    Called by the repr() built-in function and by reverse quotes.

      */
    /* Implemented elsewhere:

    PyObject *PyObject_Str(PyObject *o);

    Compute the string representation of object, o.  Returns the
    string representation on success, NULL on failure.  This is
    the equivalent of the Python expression: str(o).)

    Called by the str() built-in function and by the print
    statement.

      */
    /* Implemented elsewhere:

    PyObject *PyObject_Unicode(PyObject *o);

    Compute the unicode representation of object, o.  Returns the
    unicode representation on success, NULL on failure.  This is
    the equivalent of the Python expression: unistr(o).)

    Called by the unistr() built-in function.

      */
    /* Declared elsewhere

    PyAPI_FUNC(int) PyCallable_Check(PyObject *o);

    Determine if the object, o, is callable.  Return 1 if the
    object is callable and 0 otherwise.

    This function always succeeds.

      */
    #[no_mangle]
    fn PyObject_Call(
        callable_object: *mut PyObject,
        args: *mut PyObject,
        kw: *mut PyObject,
    ) -> *mut PyObject;
    /* * parse a configuration string of the form "[symbology.]config[=value]".
     * the config must match one of the recognized names.
     * the symbology, if present, must match one of the recognized names.
     * if symbology is unspecified, it will be set to 0.
     * if value is unspecified it will be set to 1.
     * @returns 0 if the config is parsed successfully, 1 otherwise
     * @since 0.4
     */
    #[no_mangle]
    fn zbar_parse_config(
        config_string: *const libc::c_char,
        symbology: *mut zbar_symbol_type_t,
        config: *mut zbar_config_t,
        value: *mut libc::c_int,
    ) -> libc::c_int;
    /* * return user specified data value associated with the image.
     * @since 0.5
     */
    #[no_mangle]
    fn zbar_image_get_userdata(image: *const zbar_image_t) -> *mut libc::c_void;
    /* * constructor.
     * if threaded is set and threading is available the processor
     * will spawn threads where appropriate to avoid blocking and
     * improve responsiveness
     */
    #[no_mangle]
    fn zbar_processor_create(threaded: libc::c_int) -> *mut zbar_processor_t;
    /* * destructor.  cleans up all resources associated with the processor
     */
    #[no_mangle]
    fn zbar_processor_destroy(processor: *mut zbar_processor_t);
    /* * (re)initialization.
     * opens a video input device and/or prepares to display output
     */
    #[no_mangle]
    fn zbar_processor_init(
        processor: *mut zbar_processor_t,
        video_device: *const libc::c_char,
        enable_display: libc::c_int,
    ) -> libc::c_int;
    /* * request a preferred size for the video image from the device.
     * the request may be adjusted or completely ignored by the driver.
     * @note must be called before zbar_processor_init()
     * @since 0.6
     */
    #[no_mangle]
    fn zbar_processor_request_size(
        processor: *mut zbar_processor_t,
        width: libc::c_uint,
        height: libc::c_uint,
    ) -> libc::c_int;
    /* * setup result handler callback.
     * the specified function will be called by the processor whenever
     * new results are available from the video stream or a static image.
     * pass a NULL value to disable callbacks.
     * @param processor the object on which to set the handler.
     * @param handler the function to call when new results are available.
     * @param userdata is set as with zbar_processor_set_userdata().
     * @returns the previously registered handler
     */
    #[no_mangle]
    fn zbar_processor_set_data_handler(
        processor: *mut zbar_processor_t,
        handler: Option<zbar_image_data_handler_t>,
        userdata: *const libc::c_void,
    ) -> Option<zbar_image_data_handler_t>;
    /* * associate user specified data value with the processor.
     * @since 0.6
     */
    #[no_mangle]
    fn zbar_processor_set_userdata(processor: *mut zbar_processor_t, userdata: *mut libc::c_void);
    /* * set config for indicated symbology (0 for all) to specified value.
     * @returns 0 for success, non-0 for failure (config does not apply to
     * specified symbology, or value out of range)
     * @see zbar_decoder_set_config()
     * @since 0.4
     */
    #[no_mangle]
    fn zbar_processor_set_config(
        processor: *mut zbar_processor_t,
        symbology: zbar_symbol_type_t,
        config: zbar_config_t,
        value: libc::c_int,
    ) -> libc::c_int;
    /* * retrieve the current state of the ouput window.
     * @returns 1 if the output window is currently displayed, 0 if not.
     * @returns -1 if an error occurs
     */
    #[no_mangle]
    fn zbar_processor_is_visible(processor: *mut zbar_processor_t) -> libc::c_int;
    /* * show or hide the display window owned by the library.
     * the size will be adjusted to the input size
     */
    #[no_mangle]
    fn zbar_processor_set_visible(
        processor: *mut zbar_processor_t,
        visible: libc::c_int,
    ) -> libc::c_int;
    /* * control the processor in free running video mode.
     * only works if video input is initialized. if threading is in use,
     * scanning will occur in the background, otherwise this is only
     * useful wrapping calls to zbar_processor_user_wait(). if the
     * library output window is visible, video display will be enabled.
     */
    #[no_mangle]
    fn zbar_processor_set_active(
        processor: *mut zbar_processor_t,
        active: libc::c_int,
    ) -> libc::c_int;
    /* * retrieve decode results for last scanned image/frame.
     * @returns the symbol set result container or NULL if no results are
     * available
     * @note the returned symbol set has its reference count incremented;
     * ensure that the count is decremented after use
     * @since 0.10
     */
    #[no_mangle]
    fn zbar_processor_get_results(processor: *const zbar_processor_t) -> *const zbar_symbol_set_t;
    /* * wait for input to the display window from the user
     * (via mouse or keyboard).
     * @returns >0 when input is received, 0 if timeout ms expired
     * with no input or -1 in case of an error
     */
    #[no_mangle]
    fn zbar_processor_user_wait(
        processor: *mut zbar_processor_t,
        timeout: libc::c_int,
    ) -> libc::c_int;
    /* * process from the video stream until a result is available,
     * or the timeout (in milliseconds) expires.
     * specify a timeout of -1 to scan indefinitely
     * (zbar_processor_set_active() may still be used to abort the scan
     * from another thread).
     * if the library window is visible, video display will be enabled.
     * @note that multiple results may still be returned (despite the
     * name).
     * @returns >0 if symbols were successfully decoded,
     * 0 if no symbols were found (ie, the timeout expired)
     * or -1 if an error occurs
     */
    #[no_mangle]
    fn zbar_process_one(processor: *mut zbar_processor_t, timeout: libc::c_int) -> libc::c_int;
    /* * process the provided image for barcodes.
     * if the library window is visible, the image will be displayed.
     * @returns >0 if symbols were successfully decoded,
     * 0 if no symbols were found or -1 if an error occurs
     */
    #[no_mangle]
    fn zbar_process_image(
        processor: *mut zbar_processor_t,
        image: *mut zbar_image_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn zbarErr_Set(self_0: *mut PyObject) -> *mut PyObject;
    #[no_mangle]
    static mut zbarImage_Type: PyTypeObject;
    #[no_mangle]
    fn zbarImage_FromImage(zimg: *mut zbar_image_t) -> *mut zbarImage;
    #[no_mangle]
    fn zbarImage_validate(image: *mut zbarImage) -> libc::c_int;
    #[no_mangle]
    fn zbarSymbolSet_FromSymbolSet(zsyms: *const zbar_symbol_set_t) -> *mut zbarSymbolSet;
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
/* Float object interface */
/*
PyFloatObject represents a (double precision) floating point number.
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyFloatObject {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub ob_fval: libc::c_double,
}
/* Tuple object interface */
/*
Another generally useful object type is a tuple of object pointers.
For Python, this is an immutable type.  C code can change the tuple items
(but not their number), and even use tuples are general-purpose arrays of
object references, but in general only brand new tuples should be mutated,
not ones that might already have been exposed to Python code.

*** WARNING *** PyTuple_SetItem does not increment the new item's reference
count, but does decrement the reference count of the item it replaces,
if not nil.  It does *decrement* the reference count if it is *not*
inserted in the tuple.  Similarly, PyTuple_GetItem does not increment the
returned item's reference count.
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyTupleObject {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub ob_size: Py_ssize_t,
    pub ob_item: [*mut PyObject; 1],
}
/* Thread and interpreter state structures and their interfaces */
/* State shared between threads */
/* Forward */
/* Forward */
/* State unique per thread */
/* Avoid including frameobject.h */
/* Py_tracefunc return -1 when raising an exception, or 0 for success. */
/* The following values are used for 'what' for tracefunc functions: */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ts {
    pub next: *mut _ts,
    pub interp: *mut PyInterpreterState,
    pub frame: *mut _frame,
    pub recursion_depth: libc::c_int,
    pub tracing: libc::c_int,
    pub use_tracing: libc::c_int,
    pub c_profilefunc: Py_tracefunc,
    pub c_tracefunc: Py_tracefunc,
    pub c_profileobj: *mut PyObject,
    pub c_traceobj: *mut PyObject,
    pub curexc_type: *mut PyObject,
    pub curexc_value: *mut PyObject,
    pub curexc_traceback: *mut PyObject,
    pub exc_type: *mut PyObject,
    pub exc_value: *mut PyObject,
    pub exc_traceback: *mut PyObject,
    pub dict: *mut PyObject,
    pub tick_counter: libc::c_int,
    pub gilstate_counter: libc::c_int,
    pub async_exc: *mut PyObject,
    pub thread_id: libc::c_long,
    pub trash_delete_nesting: libc::c_int,
    pub trash_delete_later: *mut PyObject,
}
pub type Py_tracefunc = Option<
    unsafe extern fn(
        _: *mut PyObject,
        _: *mut _frame,
        _: libc::c_int,
        _: *mut PyObject,
    ) -> libc::c_int,
>;
pub type PyInterpreterState = _is;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _is {
    pub next: *mut _is,
    pub tstate_head: *mut _ts,
    pub modules: *mut PyObject,
    pub sysdict: *mut PyObject,
    pub builtins: *mut PyObject,
    pub modules_reloading: *mut PyObject,
    pub codec_search_path: *mut PyObject,
    pub codec_search_cache: *mut PyObject,
    pub codec_error_registry: *mut PyObject,
    pub dlopenflags: libc::c_int,
}
pub type PyThreadState = _ts;
pub type PyGILState_STATE = libc::c_uint;
pub const PyGILState_UNLOCKED: PyGILState_STATE = 1;
pub const PyGILState_LOCKED: PyGILState_STATE = 0;
/* XXX signal handlers should also be here */
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
pub type zbar_config_t = zbar_config_e;
pub type zbar_symbol_set_t = zbar_symbol_set_s;
/* * opaque image object. */
pub type zbar_image_t = zbar_image_s;
/* * data handler callback function.
 * called when decoded symbol results are available for an image
 */
pub type zbar_image_data_handler_t =
    unsafe extern fn(_: *mut zbar_image_t, _: *const libc::c_void) -> ();
/* * opaque standalone processor object. */
pub type zbar_processor_t = zbar_processor_s;
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
pub struct zbarSymbolSet {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub zsyms: *const zbar_symbol_set_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zbarProcessor {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub zproc: *mut zbar_processor_t,
    pub handler: *mut PyObject,
    pub closure: *mut PyObject,
}
/* * parse configuration string using zbar_parse_config()
 * and apply to processor using zbar_processor_set_config().
 * @returns 0 for success, non-0 for failure
 * @see zbar_parse_config()
 * @see zbar_processor_set_config()
 * @since 0.4
 */
#[inline]
unsafe extern fn zbar_processor_parse_config(
    mut processor: *mut zbar_processor_t,
    mut config_string: *const libc::c_char,
) -> libc::c_int {
    let mut sym: zbar_symbol_type_t = ZBAR_NONE;
    let mut cfg: zbar_config_t = ZBAR_CFG_ENABLE;
    let mut val: libc::c_int = 0;
    return (zbar_parse_config(config_string, &mut sym, &mut cfg, &mut val) != 0
        || zbar_processor_set_config(processor, sym, cfg, val) != 0) as libc::c_int;
}
/*------------------------------------------------------------------------
 *  Copyright 2009 (c) Jeff Brown <spadix@users.sourceforge.net>
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
static mut processor_doc: [libc::c_char; 55] = [
    108, 111, 119, 32, 108, 101, 118, 101, 108, 32, 100, 101, 99, 111, 100, 101, 32, 111, 102, 32,
    109, 101, 97, 115, 117, 114, 101, 100, 32, 98, 97, 114, 47, 115, 112, 97, 99, 101, 32, 119,
    105, 100, 116, 104, 115, 46, 10, 10, 70, 73, 88, 77, 69, 46, 0,
];
unsafe extern fn processor_new(
    mut type_0: *mut PyTypeObject,
    mut args: *mut PyObject,
    mut kwds: *mut PyObject,
) -> *mut zbarProcessor {
    static mut kwlist: [*mut libc::c_char; 2] = [
        b"enable_threads\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
    ];
    let mut threaded: libc::c_int = -(1 as libc::c_int);
    if PyArg_ParseTupleAndKeywords(
        args,
        kwds,
        b"|O&\x00" as *const u8 as *const libc::c_char,
        kwlist.as_mut_ptr(),
        Some(
            object_to_bool
                as unsafe extern fn(_: *mut PyObject, _: *mut libc::c_int) -> libc::c_int,
        ),
        &mut threaded as *mut libc::c_int,
    ) == 0
    {
        return 0 as *mut zbarProcessor;
    }
    /* the processor creates a thread that calls back into python,
     * so we must ensure that threads are initialized before attempting
     * to manipulate the GIL (bug #3349199)
     */
    PyEval_InitThreads();
    let mut self_0: *mut zbarProcessor = (*type_0).tp_alloc.expect("non-null function pointer")(
        type_0,
        0 as libc::c_int as Py_ssize_t,
    ) as *mut zbarProcessor;
    if self_0.is_null() {
        return 0 as *mut zbarProcessor;
    }
    (*self_0).zproc = zbar_processor_create(threaded);
    zbar_processor_set_userdata((*self_0).zproc, self_0 as *mut libc::c_void);
    if (*self_0).zproc.is_null() {
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
        return 0 as *mut zbarProcessor;
    }
    return self_0;
}
unsafe extern fn processor_traverse(
    mut self_0: *mut zbarProcessor,
    mut visit: visitproc,
    mut arg: *mut libc::c_void,
) -> libc::c_int {
    if !(*self_0).handler.is_null() {
        let mut vret: libc::c_int =
            visit.expect("non-null function pointer")((*self_0).handler, arg);
        if vret != 0 {
            return vret;
        }
    }
    if !(*self_0).closure.is_null() {
        let mut vret_0: libc::c_int =
            visit.expect("non-null function pointer")((*self_0).closure, arg);
        if vret_0 != 0 {
            return vret_0;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern fn processor_clear(mut self_0: *mut zbarProcessor) -> libc::c_int {
    zbar_processor_set_data_handler((*self_0).zproc, None, 0 as *const libc::c_void);
    zbar_processor_set_userdata((*self_0).zproc, 0 as *mut libc::c_void);
    if !(*self_0).handler.is_null() {
        let mut _py_tmp: *mut PyObject = (*self_0).handler;
        (*self_0).handler = 0 as *mut PyObject;
        (*_py_tmp).ob_refcnt -= 1;
        if !((*_py_tmp).ob_refcnt != 0 as libc::c_int as libc::c_long) {
            Some((*(*_py_tmp).ob_type).tp_dealloc.expect("non-null function pointer"))
                .expect("non-null function pointer")(_py_tmp);
        }
    }
    if !(*self_0).closure.is_null() {
        let mut _py_tmp_0: *mut PyObject = (*self_0).closure;
        (*self_0).closure = 0 as *mut PyObject;
        (*_py_tmp_0).ob_refcnt -= 1;
        if !((*_py_tmp_0).ob_refcnt != 0 as libc::c_int as libc::c_long) {
            Some((*(*_py_tmp_0).ob_type).tp_dealloc.expect("non-null function pointer"))
                .expect("non-null function pointer")(_py_tmp_0);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern fn processor_dealloc(mut self_0: *mut zbarProcessor) {
    processor_clear(self_0);
    zbar_processor_destroy((*self_0).zproc);
    (*(*(self_0 as *mut PyObject)).ob_type).tp_free.expect("non-null function pointer")(
        self_0 as *mut PyObject as *mut libc::c_void,
    );
}
unsafe extern fn processor_get_bool(
    mut self_0: *mut zbarProcessor,
    mut closure: *mut libc::c_void,
) -> *mut PyObject {
    let mut val: libc::c_int = 0;
    match closure as intptr_t {
        0 => val = zbar_processor_is_visible((*self_0).zproc),
        _ => return 0 as *mut PyObject,
    }
    if val < 0 as libc::c_int {
        return zbarErr_Set(self_0 as *mut PyObject);
    }
    return PyBool_FromLong(val as libc::c_long);
}
unsafe extern fn processor_set_bool(
    mut self_0: *mut zbarProcessor,
    mut value: *mut PyObject,
    mut closure: *mut libc::c_void,
) -> libc::c_int {
    if value.is_null() {
        PyErr_SetString(
            PyExc_TypeError,
            b"cannot delete attribute\x00" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    let mut rc: libc::c_int = 0;
    let mut val: libc::c_int = PyObject_IsTrue(value);
    if val < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    match closure as intptr_t {
        0 => rc = zbar_processor_set_visible((*self_0).zproc, val),
        1 => rc = zbar_processor_set_active((*self_0).zproc, val),
        _ => return -(1 as libc::c_int),
    }
    if rc < 0 as libc::c_int {
        zbarErr_Set(self_0 as *mut PyObject);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern fn processor_get_results(
    mut self_0: *mut zbarProcessor,
    mut closure: *mut libc::c_void,
) -> *mut zbarSymbolSet {
    let mut zsyms: *const zbar_symbol_set_t = zbar_processor_get_results((*self_0).zproc);
    return zbarSymbolSet_FromSymbolSet(zsyms);
}
unsafe extern fn processor_set_request_size(
    mut self_0: *mut zbarProcessor,
    mut value: *mut PyObject,
    mut closure: *mut libc::c_void,
) -> libc::c_int {
    if value.is_null() {
        zbar_processor_request_size(
            (*self_0).zproc,
            0 as libc::c_int as libc::c_uint,
            0 as libc::c_int as libc::c_uint,
        );
        return 0 as libc::c_int;
    }
    let mut dims: [libc::c_int; 2] = [0; 2];
    if parse_dimensions(value, dims.as_mut_ptr(), 2 as libc::c_int) != 0
        || dims[0 as libc::c_int as usize] < 0 as libc::c_int
        || dims[1 as libc::c_int as usize] < 0 as libc::c_int
    {
        PyErr_SetString(
            PyExc_ValueError,
            b"request_size must be a sequence of two positive ints\x00" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    zbar_processor_request_size(
        (*self_0).zproc,
        dims[0 as libc::c_int as usize] as libc::c_uint,
        dims[1 as libc::c_int as usize] as libc::c_uint,
    );
    return 0 as libc::c_int;
}
static mut processor_getset: [PyGetSetDef; 5] = unsafe {
    [
        {
            let mut init = PyGetSetDef {
                name: b"visible\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                get: ::std::mem::transmute::<
                    Option<
                        unsafe extern fn(
                            _: *mut zbarProcessor,
                            _: *mut libc::c_void,
                        ) -> *mut PyObject,
                    >,
                    getter,
                >(Some(
                    processor_get_bool
                        as unsafe extern fn(
                            _: *mut zbarProcessor,
                            _: *mut libc::c_void,
                        ) -> *mut PyObject,
                )),
                set: ::std::mem::transmute::<
                    Option<
                        unsafe extern fn(
                            _: *mut zbarProcessor,
                            _: *mut PyObject,
                            _: *mut libc::c_void,
                        ) -> libc::c_int,
                    >,
                    setter,
                >(Some(
                    processor_set_bool
                        as unsafe extern fn(
                            _: *mut zbarProcessor,
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
                name: b"active\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                get: None,
                set: ::std::mem::transmute::<
                    Option<
                        unsafe extern fn(
                            _: *mut zbarProcessor,
                            _: *mut PyObject,
                            _: *mut libc::c_void,
                        ) -> libc::c_int,
                    >,
                    setter,
                >(Some(
                    processor_set_bool
                        as unsafe extern fn(
                            _: *mut zbarProcessor,
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
                name: b"results\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                get: ::std::mem::transmute::<
                    Option<
                        unsafe extern fn(
                            _: *mut zbarProcessor,
                            _: *mut libc::c_void,
                        ) -> *mut zbarSymbolSet,
                    >,
                    getter,
                >(Some(
                    processor_get_results
                        as unsafe extern fn(
                            _: *mut zbarProcessor,
                            _: *mut libc::c_void,
                        ) -> *mut zbarSymbolSet,
                )),
                set: None,
                doc: 0 as *const libc::c_char as *mut libc::c_char,
                closure: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = PyGetSetDef {
                name: b"request_size\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                get: None,
                set: ::std::mem::transmute::<
                    Option<
                        unsafe extern fn(
                            _: *mut zbarProcessor,
                            _: *mut PyObject,
                            _: *mut libc::c_void,
                        ) -> libc::c_int,
                    >,
                    setter,
                >(Some(
                    processor_set_request_size
                        as unsafe extern fn(
                            _: *mut zbarProcessor,
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
unsafe extern fn processor_set_config(
    mut self_0: *mut zbarProcessor,
    mut args: *mut PyObject,
    mut kwds: *mut PyObject,
) -> *mut PyObject {
    let mut sym: zbar_symbol_type_t = ZBAR_NONE;
    let mut cfg: zbar_config_t = ZBAR_CFG_ENABLE;
    let mut val: libc::c_int = 1 as libc::c_int;
    static mut kwlist: [*mut libc::c_char; 4] = [
        b"symbology\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"config\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"value\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
    ];
    if PyArg_ParseTupleAndKeywords(
        args,
        kwds,
        b"|iii\x00" as *const u8 as *const libc::c_char,
        kwlist.as_mut_ptr(),
        &mut sym as *mut zbar_symbol_type_t,
        &mut cfg as *mut zbar_config_t,
        &mut val as *mut libc::c_int,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    if zbar_processor_set_config((*self_0).zproc, sym, cfg, val) != 0 {
        PyErr_SetString(
            PyExc_ValueError,
            b"invalid configuration setting\x00" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut PyObject;
    }
    let ref mut fresh1 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
    *fresh1 += 1;
    return &mut _Py_NoneStruct;
}
unsafe extern fn processor_init_(
    mut self_0: *mut zbarProcessor,
    mut args: *mut PyObject,
    mut kwds: *mut PyObject,
) -> *mut PyObject {
    let mut dev: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
    let mut disp: libc::c_int = 1 as libc::c_int;
    static mut kwlist: [*mut libc::c_char; 3] = [
        b"video_device\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"enable_display\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
    ];
    if PyArg_ParseTupleAndKeywords(
        args,
        kwds,
        b"|zO&\x00" as *const u8 as *const libc::c_char,
        kwlist.as_mut_ptr(),
        &mut dev as *mut *const libc::c_char,
        Some(
            object_to_bool
                as unsafe extern fn(_: *mut PyObject, _: *mut libc::c_int) -> libc::c_int,
        ),
        &mut disp as *mut libc::c_int,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    if zbar_processor_init((*self_0).zproc, dev, disp) != 0 {
        return zbarErr_Set(self_0 as *mut PyObject);
    }
    let ref mut fresh2 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
    *fresh2 += 1;
    return &mut _Py_NoneStruct;
}
unsafe extern fn processor_parse_config(
    mut self_0: *mut zbarProcessor,
    mut args: *mut PyObject,
    mut kwds: *mut PyObject,
) -> *mut PyObject {
    let mut cfg: *const libc::c_char = 0 as *const libc::c_char;
    static mut kwlist: [*mut libc::c_char; 2] = [
        b"config\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
    ];
    if PyArg_ParseTupleAndKeywords(
        args,
        kwds,
        b"s\x00" as *const u8 as *const libc::c_char,
        kwlist.as_mut_ptr(),
        &mut cfg as *mut *const libc::c_char,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    if zbar_processor_parse_config((*self_0).zproc, cfg) != 0 {
        PyErr_Format(
            PyExc_ValueError,
            b"invalid configuration setting: %s\x00" as *const u8 as *const libc::c_char,
            cfg,
        );
        return 0 as *mut PyObject;
    }
    let ref mut fresh3 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
    *fresh3 += 1;
    return &mut _Py_NoneStruct;
}
unsafe extern fn object_to_timeout(
    mut obj: *mut PyObject,
    mut val: *mut libc::c_int,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    if (*obj).ob_type == &mut PyFloat_Type as *mut PyTypeObject
        || PyType_IsSubtype((*obj).ob_type, &mut PyFloat_Type) != 0
    {
        tmp = ((*(obj as *mut PyFloatObject)).ob_fval * 1000 as libc::c_int as libc::c_double)
            as libc::c_int
    } else {
        tmp = (PyInt_AsLong(obj) * 1000 as libc::c_int as libc::c_long) as libc::c_int
    }
    if tmp < 0 as libc::c_int && !PyErr_Occurred().is_null() {
        return 0 as libc::c_int;
    }
    *val = tmp;
    return 1 as libc::c_int;
}
unsafe extern fn processor_user_wait(
    mut self_0: *mut zbarProcessor,
    mut args: *mut PyObject,
    mut kwds: *mut PyObject,
) -> *mut PyObject {
    let mut timeout: libc::c_int = -(1 as libc::c_int);
    static mut kwlist: [*mut libc::c_char; 2] = [
        b"timeout\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
    ];
    if PyArg_ParseTupleAndKeywords(
        args,
        kwds,
        b"|O&\x00" as *const u8 as *const libc::c_char,
        kwlist.as_mut_ptr(),
        Some(
            object_to_timeout
                as unsafe extern fn(_: *mut PyObject, _: *mut libc::c_int) -> libc::c_int,
        ),
        &mut timeout as *mut libc::c_int,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    let mut rc: libc::c_int = -(1 as libc::c_int);
    let mut _save: *mut PyThreadState = 0 as *mut PyThreadState;
    _save = PyEval_SaveThread();
    rc = zbar_processor_user_wait((*self_0).zproc, timeout);
    PyEval_RestoreThread(_save);
    if rc < 0 as libc::c_int {
        return zbarErr_Set(self_0 as *mut PyObject);
    }
    return PyInt_FromLong(rc as libc::c_long);
}
unsafe extern fn processor_process_one(
    mut self_0: *mut zbarProcessor,
    mut args: *mut PyObject,
    mut kwds: *mut PyObject,
) -> *mut PyObject {
    let mut timeout: libc::c_int = -(1 as libc::c_int);
    static mut kwlist: [*mut libc::c_char; 2] = [
        b"timeout\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
    ];
    if PyArg_ParseTupleAndKeywords(
        args,
        kwds,
        b"|O&\x00" as *const u8 as *const libc::c_char,
        kwlist.as_mut_ptr(),
        Some(
            object_to_timeout
                as unsafe extern fn(_: *mut PyObject, _: *mut libc::c_int) -> libc::c_int,
        ),
        &mut timeout as *mut libc::c_int,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    let mut rc: libc::c_int = -(1 as libc::c_int);
    let mut _save: *mut PyThreadState = 0 as *mut PyThreadState;
    _save = PyEval_SaveThread();
    rc = zbar_process_one((*self_0).zproc, timeout);
    PyEval_RestoreThread(_save);
    if rc < 0 as libc::c_int {
        return zbarErr_Set(self_0 as *mut PyObject);
    }
    return PyInt_FromLong(rc as libc::c_long);
}
unsafe extern fn processor_process_image(
    mut self_0: *mut zbarProcessor,
    mut args: *mut PyObject,
    mut kwds: *mut PyObject,
) -> *mut PyObject {
    let mut img: *mut zbarImage = 0 as *mut zbarImage;
    static mut kwlist: [*mut libc::c_char; 2] = [
        b"image\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
    ];
    if PyArg_ParseTupleAndKeywords(
        args,
        kwds,
        b"O!\x00" as *const u8 as *const libc::c_char,
        kwlist.as_mut_ptr(),
        &mut zbarImage_Type as *mut PyTypeObject,
        &mut img as *mut *mut zbarImage,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    if zbarImage_validate(img) != 0 {
        return 0 as *mut PyObject;
    }
    let mut n: libc::c_int = -(1 as libc::c_int);
    let mut _save: *mut PyThreadState = 0 as *mut PyThreadState;
    _save = PyEval_SaveThread();
    n = zbar_process_image((*self_0).zproc, (*img).zimg);
    PyEval_RestoreThread(_save);
    if n < 0 as libc::c_int {
        return zbarErr_Set(self_0 as *mut PyObject);
    }
    return PyInt_FromLong(n as libc::c_long);
}
#[no_mangle]
pub unsafe extern fn process_handler(
    mut zimg: *mut zbar_image_t,
    mut userdata: *const libc::c_void,
) {
    let mut args: *mut PyObject = 0 as *mut PyObject;
    let mut junk: *mut PyObject = 0 as *mut PyObject;
    let mut current_block: u64;
    let mut gstate: PyGILState_STATE = PyGILState_LOCKED;
    gstate = PyGILState_Ensure();
    let mut self_0: *mut zbarProcessor = userdata as *mut zbarProcessor;
    let mut img: *mut zbarImage = zbar_image_get_userdata(zimg) as *mut zbarImage;
    if img.is_null() || (*img).zimg != zimg {
        img = zbarImage_FromImage(zimg);
        if img.is_null() {
            PyErr_NoMemory();
            current_block = 1452600018190894927;
        } else {
            current_block = 2868539653012386629;
        }
    } else {
        let ref mut fresh4 = (*(img as *mut PyObject)).ob_refcnt;
        *fresh4 += 1;
        current_block = 2868539653012386629;
    }
    match current_block {
        2868539653012386629 => {
            args = PyTuple_New(3 as libc::c_int as Py_ssize_t);
            let ref mut fresh5 = (*(self_0 as *mut PyObject)).ob_refcnt;
            *fresh5 += 1;
            (*(*self_0).closure).ob_refcnt += 1;
            let ref mut fresh6 = *(*(args as *mut PyTupleObject))
                .ob_item
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize);
            *fresh6 = self_0 as *mut PyObject;
            let ref mut fresh7 = *(*(args as *mut PyTupleObject))
                .ob_item
                .as_mut_ptr()
                .offset(1 as libc::c_int as isize);
            *fresh7 = img as *mut PyObject;
            let ref mut fresh8 = *(*(args as *mut PyTupleObject))
                .ob_item
                .as_mut_ptr()
                .offset(2 as libc::c_int as isize);
            *fresh8 = (*self_0).closure;
            junk = PyObject_Call((*self_0).handler, args, 0 as *mut PyObject);
            if !junk.is_null() {
                (*junk).ob_refcnt -= 1;
                if !((*junk).ob_refcnt != 0 as libc::c_int as libc::c_long) {
                    Some((*(*junk).ob_type).tp_dealloc.expect("non-null function pointer"))
                        .expect("non-null function pointer")(junk);
                }
            } else {
                PySys_WriteStderr(
                    b"in ZBar Processor data_handler:\n\x00" as *const u8 as *const libc::c_char,
                );
                PyErr_Print();
            }
            (*args).ob_refcnt -= 1;
            if !((*args).ob_refcnt != 0 as libc::c_int as libc::c_long) {
                Some((*(*args).ob_type).tp_dealloc.expect("non-null function pointer"))
                    .expect("non-null function pointer")(args);
            }
        }
        _ => {}
    }
    PyGILState_Release(gstate);
}
unsafe extern fn processor_set_data_handler(
    mut self_0: *mut zbarProcessor,
    mut args: *mut PyObject,
    mut kwds: *mut PyObject,
) -> *mut PyObject {
    let mut handler: *mut PyObject = &mut _Py_NoneStruct;
    let mut closure: *mut PyObject = &mut _Py_NoneStruct;
    static mut kwlist: [*mut libc::c_char; 3] = [
        b"handler\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"closure\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
    ];
    if PyArg_ParseTupleAndKeywords(
        args,
        kwds,
        b"|OO\x00" as *const u8 as *const libc::c_char,
        kwlist.as_mut_ptr(),
        &mut handler as *mut *mut PyObject,
        &mut closure as *mut *mut PyObject,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    if handler != &mut _Py_NoneStruct as *mut PyObject && PyCallable_Check(handler) == 0 {
        PyErr_Format(
            PyExc_ValueError,
            b"handler %.50s is not callable\x00" as *const u8 as *const libc::c_char,
            (*(*handler).ob_type).tp_name,
        );
        return 0 as *mut PyObject;
    }
    if !(*self_0).handler.is_null() {
        let mut _py_tmp: *mut PyObject = (*self_0).handler;
        (*self_0).handler = 0 as *mut PyObject;
        (*_py_tmp).ob_refcnt -= 1;
        if !((*_py_tmp).ob_refcnt != 0 as libc::c_int as libc::c_long) {
            Some((*(*_py_tmp).ob_type).tp_dealloc.expect("non-null function pointer"))
                .expect("non-null function pointer")(_py_tmp);
        }
    }
    if !(*self_0).closure.is_null() {
        let mut _py_tmp_0: *mut PyObject = (*self_0).closure;
        (*self_0).closure = 0 as *mut PyObject;
        (*_py_tmp_0).ob_refcnt -= 1;
        if !((*_py_tmp_0).ob_refcnt != 0 as libc::c_int as libc::c_long) {
            Some((*(*_py_tmp_0).ob_type).tp_dealloc.expect("non-null function pointer"))
                .expect("non-null function pointer")(_py_tmp_0);
        }
    }
    if handler != &mut _Py_NoneStruct as *mut PyObject {
        (*handler).ob_refcnt += 1;
        (*self_0).handler = handler;
        (*closure).ob_refcnt += 1;
        (*self_0).closure = closure;
        zbar_processor_set_data_handler(
            (*self_0).zproc,
            Some(
                process_handler
                    as unsafe extern fn(_: *mut zbar_image_t, _: *const libc::c_void) -> (),
            ),
            self_0 as *const libc::c_void,
        );
    } else {
        (*self_0).closure = 0 as *mut PyObject;
        (*self_0).handler = (*self_0).closure;
        zbar_processor_set_data_handler((*self_0).zproc, None, self_0 as *const libc::c_void);
    }
    let ref mut fresh9 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
    *fresh9 += 1;
    return &mut _Py_NoneStruct;
}
static mut processor_methods: [PyMethodDef; 8] = unsafe {
    [
        {
            let mut init = PyMethodDef {
                ml_name: b"init\x00" as *const u8 as *const libc::c_char,
                ml_meth: ::std::mem::transmute::<
                    Option<
                        unsafe extern fn(
                            _: *mut zbarProcessor,
                            _: *mut PyObject,
                            _: *mut PyObject,
                        ) -> *mut PyObject,
                    >,
                    PyCFunction,
                >(Some(
                    processor_init_
                        as unsafe extern fn(
                            _: *mut zbarProcessor,
                            _: *mut PyObject,
                            _: *mut PyObject,
                        ) -> *mut PyObject,
                )),
                ml_flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"set_config\x00" as *const u8 as *const libc::c_char,
                ml_meth: ::std::mem::transmute::<
                    Option<
                        unsafe extern fn(
                            _: *mut zbarProcessor,
                            _: *mut PyObject,
                            _: *mut PyObject,
                        ) -> *mut PyObject,
                    >,
                    PyCFunction,
                >(Some(
                    processor_set_config
                        as unsafe extern fn(
                            _: *mut zbarProcessor,
                            _: *mut PyObject,
                            _: *mut PyObject,
                        ) -> *mut PyObject,
                )),
                ml_flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"parse_config\x00" as *const u8 as *const libc::c_char,
                ml_meth: ::std::mem::transmute::<
                    Option<
                        unsafe extern fn(
                            _: *mut zbarProcessor,
                            _: *mut PyObject,
                            _: *mut PyObject,
                        ) -> *mut PyObject,
                    >,
                    PyCFunction,
                >(Some(
                    processor_parse_config
                        as unsafe extern fn(
                            _: *mut zbarProcessor,
                            _: *mut PyObject,
                            _: *mut PyObject,
                        ) -> *mut PyObject,
                )),
                ml_flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"user_wait\x00" as *const u8 as *const libc::c_char,
                ml_meth: ::std::mem::transmute::<
                    Option<
                        unsafe extern fn(
                            _: *mut zbarProcessor,
                            _: *mut PyObject,
                            _: *mut PyObject,
                        ) -> *mut PyObject,
                    >,
                    PyCFunction,
                >(Some(
                    processor_user_wait
                        as unsafe extern fn(
                            _: *mut zbarProcessor,
                            _: *mut PyObject,
                            _: *mut PyObject,
                        ) -> *mut PyObject,
                )),
                ml_flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"process_one\x00" as *const u8 as *const libc::c_char,
                ml_meth: ::std::mem::transmute::<
                    Option<
                        unsafe extern fn(
                            _: *mut zbarProcessor,
                            _: *mut PyObject,
                            _: *mut PyObject,
                        ) -> *mut PyObject,
                    >,
                    PyCFunction,
                >(Some(
                    processor_process_one
                        as unsafe extern fn(
                            _: *mut zbarProcessor,
                            _: *mut PyObject,
                            _: *mut PyObject,
                        ) -> *mut PyObject,
                )),
                ml_flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"process_image\x00" as *const u8 as *const libc::c_char,
                ml_meth: ::std::mem::transmute::<
                    Option<
                        unsafe extern fn(
                            _: *mut zbarProcessor,
                            _: *mut PyObject,
                            _: *mut PyObject,
                        ) -> *mut PyObject,
                    >,
                    PyCFunction,
                >(Some(
                    processor_process_image
                        as unsafe extern fn(
                            _: *mut zbarProcessor,
                            _: *mut PyObject,
                            _: *mut PyObject,
                        ) -> *mut PyObject,
                )),
                ml_flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"set_data_handler\x00" as *const u8 as *const libc::c_char,
                ml_meth: ::std::mem::transmute::<
                    Option<
                        unsafe extern fn(
                            _: *mut zbarProcessor,
                            _: *mut PyObject,
                            _: *mut PyObject,
                        ) -> *mut PyObject,
                    >,
                    PyCFunction,
                >(Some(
                    processor_set_data_handler
                        as unsafe extern fn(
                            _: *mut zbarProcessor,
                            _: *mut PyObject,
                            _: *mut PyObject,
                        ) -> *mut PyObject,
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
pub static mut zbarProcessor_Type: PyTypeObject = unsafe {
    {
        let mut init = _typeobject {
            ob_refcnt: 1 as libc::c_int as Py_ssize_t,
            ob_type: 0 as *const _typeobject as *mut _typeobject,
            ob_size: 0,
            tp_name: b"zbar.Processor\x00" as *const u8 as *const libc::c_char,
            tp_basicsize: ::std::mem::size_of::<zbarProcessor>() as libc::c_ulong as Py_ssize_t,
            tp_itemsize: 0,
            tp_dealloc: ::std::mem::transmute::<
                Option<unsafe extern fn(_: *mut zbarProcessor) -> ()>,
                destructor,
            >(Some(
                processor_dealloc as unsafe extern fn(_: *mut zbarProcessor) -> (),
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
            tp_doc: processor_doc.as_ptr() as *mut _,
            tp_traverse: ::std::mem::transmute::<
                Option<
                    unsafe extern fn(
                        _: *mut zbarProcessor,
                        _: visitproc,
                        _: *mut libc::c_void,
                    ) -> libc::c_int,
                >,
                traverseproc,
            >(Some(
                processor_traverse
                    as unsafe extern fn(
                        _: *mut zbarProcessor,
                        _: visitproc,
                        _: *mut libc::c_void,
                    ) -> libc::c_int,
            )),
            tp_clear: ::std::mem::transmute::<
                Option<unsafe extern fn(_: *mut zbarProcessor) -> libc::c_int>,
                inquiry,
            >(Some(
                processor_clear as unsafe extern fn(_: *mut zbarProcessor) -> libc::c_int,
            )),
            tp_richcompare: None,
            tp_weaklistoffset: 0,
            tp_iter: None,
            tp_iternext: None,
            tp_methods: processor_methods.as_ptr() as *mut _,
            tp_members: 0 as *const PyMemberDef as *mut PyMemberDef,
            tp_getset: processor_getset.as_ptr() as *mut _,
            tp_base: 0 as *const _typeobject as *mut _typeobject,
            tp_dict: 0 as *const PyObject as *mut PyObject,
            tp_descr_get: None,
            tp_descr_set: None,
            tp_dictoffset: 0,
            tp_init: None,
            tp_alloc: None,
            tp_new: ::std::mem::transmute::<
                Option<
                    unsafe extern fn(
                        _: *mut PyTypeObject,
                        _: *mut PyObject,
                        _: *mut PyObject,
                    ) -> *mut zbarProcessor,
                >,
                newfunc,
            >(Some(
                processor_new
                    as unsafe extern fn(
                        _: *mut PyTypeObject,
                        _: *mut PyObject,
                        _: *mut PyObject,
                    ) -> *mut zbarProcessor,
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
