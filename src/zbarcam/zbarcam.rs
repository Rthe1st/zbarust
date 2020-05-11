use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type zbar_symbol_s;
    pub type zbar_image_s;
    pub type zbar_processor_s;
    #[no_mangle]
    fn strtol(__nptr: *const libc::c_char, __endptr: *mut *mut libc::c_char,
              __base: libc::c_int) -> libc::c_long;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fwrite(__ptr: *const libc::c_void, __size: size_t, __n: size_t,
              __s: *mut FILE) -> size_t;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
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
    /* *< light area or space between bars */
    /* *< dark area or colored bar segment */
    /* * decoded symbol type. */
    /* *< no symbol decoded */
    /* *< intermediate status */
    /* *< GS1 2-digit add-on */
    /* *< GS1 5-digit add-on */
    /* *< EAN-8 */
    /* *< UPC-E */
    /* *< ISBN-10 (from EAN-13). @since 0.4 */
    /* *< UPC-A */
    /* *< EAN-13 */
    /* *< ISBN-13 (from EAN-13). @since 0.4 */
    /* *< EAN/UPC composite */
    /* *< Interleaved 2 of 5. @since 0.4 */
    /* *< GS1 DataBar (RSS). @since 0.11 */
    /* *< GS1 DataBar Expanded. @since 0.11 */
    /* *< Codabar. @since 0.11 */
    /* *< Code 39. @since 0.4 */
    /* *< PDF417. @since 0.6 */
    /* *< QR Code. @since 0.10 */
    /* *< Code 93. @since 0.11 */
    /* *< Code 128 */
    /* * mask for base symbol type.
     * @deprecated in 0.11, remove this from existing code
     */
    /* * 2-digit add-on flag.
     * @deprecated in 0.11, a ::ZBAR_EAN2 component is used for
     * 2-digit GS1 add-ons
     */
    /* * 5-digit add-on flag.
     * @deprecated in 0.11, a ::ZBAR_EAN5 component is used for
     * 5-digit GS1 add-ons
     */
    /* * add-on flag mask.
     * @deprecated in 0.11, GS1 add-ons are represented using composite
     * symbols of type ::ZBAR_COMPOSITE; add-on components use ::ZBAR_EAN2
     * or ::ZBAR_EAN5
     */
    /* * decoded symbol coarse orientation.
 * @since 0.11
 */
    /* *< unable to determine orientation */
    /* *< upright, read left to right */
    /* *< sideways, read top to bottom */
    /* *< upside-down, read right to left */
    /* *< sideways, read bottom to top */
    /* * error codes. */
    /* *< no error */
    /* *< out of memory */
    /* *< internal library error */
    /* *< unsupported request */
    /* *< invalid request */
    /* *< system error */
    /* *< locking error */
    /* *< all resources busy */
    /* *< X11 display error */
    /* *< X11 protocol error */
    /* *< output window is closed */
    /* *< windows system error */
    /* *< number of error codes */
    /* * decoder configuration options.
 * @since 0.4
 */
    /* *< enable symbology/feature */
    /* *< enable check digit when optional */
    /* *< return check digit when present */
    /* *< enable full ASCII character set */
    /* *< number of boolean decoder configs */
    /* *< minimum data length for valid decode */
    /* *< maximum data length for valid decode */
    /* *< required video consistency frames */
    /* *< enable scanner to collect position data */
    /* *< image scanner vertical scan density */
    /* *< image scanner horizontal scan density */
    /* * decoder symbology modifier flags.
 * @since 0.11
 */
    /* * barcode tagged as GS1 (EAN.UCC) reserved
     * (eg, FNC1 before first data character).
     * data may be parsed as a sequence of GS1 AIs
     */
    /* * barcode tagged as AIM reserved
     * (eg, FNC1 after first character or digit pair)
     */
    /* * number of modifiers */
    /* * store video control menu
 * @param name name of the menu item
 * @param val integer value associated with the item
 */
    /* * store video controls
 * @param name name of the control
 * @param group name of the control group/class
 * @param type type of the control
 * @param min minimum value of control (if control is integer)
 * @param max maximum value of control (if control is integer)
 * @param def default value of control (if control is integer)
 * @param step increment steps (if control is integer)
 * @param menu menu array
 * @param menu_size menu size
 * @since 0.20
 */
    // video drivers may add extra private data in the end of this struct
    /* * retrieve runtime library version information.
 * @param major set to the running major version (unless NULL)
 * @param minor set to the running minor version (unless NULL)
 * @returns 0
 */
    /* * set global library debug level.
 * @param verbosity desired debug level.  higher values create more spew
 */
    /* * increase global library debug level.
 * eg, for -vvvv
 */
    /* * retrieve string name for symbol encoding.
 * @param sym symbol type encoding
 * @returns the static string name for the specified symbol type,
 * or "UNKNOWN" if the encoding is not recognized
 */
    /* * retrieve string name for addon encoding.
 * @param sym symbol type encoding
 * @returns static string name for any addon, or the empty string
 * if no addons were decoded
 * @deprecated in 0.11
 */
    /* * retrieve string name for configuration setting.
 * @param config setting to name
 * @returns static string name for config,
 * or the empty string if value is not a known config
 */
    /* * retrieve string name for modifier.
 * @param modifier flag to name
 * @returns static string name for modifier,
 * or the empty string if the value is not a known flag
 */
    /* * retrieve string name for orientation.
 * @param orientation orientation encoding
 * @returns the static string name for the specified orientation,
 * or "UNKNOWN" if the orientation is not recognized
 * @since 0.11
 */
    /* * parse a configuration string of the form "[symbology.]config[=value]".
 * the config must match one of the recognized names.
 * the symbology, if present, must match one of the recognized names.
 * if symbology is unspecified, it will be set to 0.
 * if value is unspecified it will be set to 1.
 * @returns 0 if the config is parsed successfully, 1 otherwise
 * @since 0.4
 */
    /* * consistently compute fourcc values across architectures
 * (adapted from v4l2 specification)
 * @since 0.11
 */
    /* * parse a fourcc string into its encoded integer value.
 * @since 0.11
 */
    /* * @internal type unsafe error API (don't use) */
    /*@}*/
    /*------------------------------------------------------------*/
/* * @name Symbol interface
 * decoded barcode symbol result object.  stores type, data, and image
 * location of decoded symbol.  all memory is owned by the library
 */
/*@{*/
    /* * @typedef zbar_symbol_t
 * opaque decoded symbol object.
 */
    /* * symbol reference count manipulation.
 * increment the reference count when you store a new reference to the
 * symbol.  decrement when the reference is no longer used.  do not
 * refer to the symbol once the count is decremented and the
 * containing image has been recycled or destroyed.
 * @note the containing image holds a reference to the symbol, so you
 * only need to use this if you keep a symbol after the image has been
 * destroyed or reused.
 * @since 0.9
 */
    /* * retrieve type of decoded symbol.
 * @returns the symbol type
 */
    /* * retrieve symbology boolean config settings.
 * @returns a bitmask indicating which configs were set for the detected
 * symbology during decoding.
 * @since 0.11
 */
    /* * retrieve symbology modifier flag settings.
 * @returns a bitmask indicating which characteristics were detected
 * during decoding.
 * @since 0.11
 */
    /* * retrieve data decoded from symbol.
 * @returns the data string
 */
    /* * retrieve length of binary data.
 * @returns the length of the decoded data
 */
    /* * retrieve a symbol confidence metric.
 * @returns an unscaled, relative quantity: larger values are better
 * than smaller values, where "large" and "small" are application
 * dependent.
 * @note expect the exact definition of this quantity to change as the
 * metric is refined.  currently, only the ordered relationship
 * between two values is defined and will remain stable in the future
 * @since 0.9
 */
    /* * retrieve current cache count.  when the cache is enabled for the
 * image_scanner this provides inter-frame reliability and redundancy
 * information for video streams.
 * @returns < 0 if symbol is still uncertain.
 * @returns 0 if symbol is newly verified.
 * @returns > 0 for duplicate symbols
 */
    /* * retrieve the number of points in the location polygon.  the
 * location polygon defines the image area that the symbol was
 * extracted from.
 * @returns the number of points in the location polygon
 * @note this is currently not a polygon, but the scan locations
 * where the symbol was decoded
 */
    /* * retrieve location polygon x-coordinates.
 * points are specified by 0-based index.
 * @returns the x-coordinate for a point in the location polygon.
 * @returns -1 if index is out of range
 */
    /* * retrieve location polygon y-coordinates.
 * points are specified by 0-based index.
 * @returns the y-coordinate for a point in the location polygon.
 * @returns -1 if index is out of range
 */
    /* * retrieve general orientation of decoded symbol.
 * @returns a coarse, axis-aligned indication of symbol orientation or
 * ::ZBAR_ORIENT_UNKNOWN if unknown
 * @since 0.11
 */
    /* * iterate the set to which this symbol belongs (there can be only one).
 * @returns the next symbol in the set, or
 * @returns NULL when no more results are available
 */
    /* * retrieve components of a composite result.
 * @returns the symbol set containing the components
 * @returns NULL if the symbol is already a physical symbol
 * @since 0.10
 */
    /* * iterate components of a composite result.
 * @returns the first physical component symbol of a composite result
 * @returns NULL if the symbol is already a physical symbol
 * @since 0.10
 */
    /* * print XML symbol element representation to user result buffer.
 * @see http://zbar.sourceforge.net/2008/barcode.xsd for the schema.
 * @param symbol is the symbol to print
 * @param buffer is the inout result pointer, it will be reallocated
 * with a larger size if necessary.
 * @param buflen is inout length of the result buffer.
 * @returns the buffer pointer
 * @since 0.6
 */
    /*@}*/
    /*------------------------------------------------------------*/
/* * @name Symbol Set interface
 * container for decoded result symbols associated with an image
 * or a composite symbol.
 * @since 0.10
 */
/*@{*/
    /* * @typedef zbar_symbol_set_t
 * opaque symbol iterator object.
 * @since 0.10
 */
    /* * reference count manipulation.
 * increment the reference count when you store a new reference.
 * decrement when the reference is no longer used.  do not refer to
 * the object any longer once references have been released.
 * @since 0.10
 */
    /* * retrieve set size.
 * @returns the number of symbols in the set.
 * @since 0.10
 */
    /* * set iterator.
 * @returns the first decoded symbol result in a set
 * @returns NULL if the set is empty
 * @since 0.10
 */
    /* * raw result iterator.
 * @returns the first decoded symbol result in a set, *before* filtering
 * @returns NULL if the set is empty
 * @since 0.11
 */
    /*@}*/
    /*------------------------------------------------------------*/
/* * @name Image interface
 * stores image data samples along with associated format and size
 * metadata
 */
/*@{*/
    /* * opaque image object. */
    /* * cleanup handler callback function.
 * called to free sample data when an image is destroyed.
 */
    /* * data handler callback function.
 * called when decoded symbol results are available for an image
 */
    /* * new image constructor.
 * @returns a new image object with uninitialized data and format.
 * this image should be destroyed (using zbar_image_destroy()) as
 * soon as the application is finished with it
 */
    /* * image destructor.  all images created by or returned to the
 * application should be destroyed using this function.  when an image
 * is destroyed, the associated data cleanup handler will be invoked
 * if available
 * @note make no assumptions about the image or the data buffer.
 * they may not be destroyed/cleaned immediately if the library
 * is still using them.  if necessary, use the cleanup handler hook
 * to keep track of image data buffers
 */
    /* * image reference count manipulation.
 * increment the reference count when you store a new reference to the
 * image.  decrement when the reference is no longer used.  do not
 * refer to the image any longer once the count is decremented.
 * zbar_image_ref(image, -1) is the same as zbar_image_destroy(image)
 * @since 0.5
 */
    /* * image format conversion.  refer to the documentation for supported
 * image formats
 * @returns a @em new image with the sample data from the original image
 * converted to the requested format.  the original image is
 * unaffected.
 * @note the converted image size may be rounded (up) due to format
 * constraints
 */
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
    /* * retrieve the image format.
 * @returns the fourcc describing the format of the image sample data
 */
    /* * retrieve a "sequence" (page/frame) number associated with this image.
 * @since 0.6
 */
    /* * retrieve the width of the image.
 * @returns the width in sample columns
 */
    /* * retrieve the height of the image.
 * @returns the height in sample rows
 */
    /* * retrieve both dimensions of the image.
 * fills in the width and height in samples
 */
    /* * retrieve the crop rectangle.
 * fills in the image coordinates of the upper left corner and size
 * of an axis-aligned rectangular area of the image that will be scanned.
 * defaults to the full image
 * @since 0.11
 */
    /* * return the image sample data.  the returned data buffer is only
 * valid until zbar_image_destroy() is called
 */
    /* * return the size of image data.
 * @since 0.6
 */
    /* * retrieve the decoded results.
 * @returns the (possibly empty) set of decoded symbols
 * @returns NULL if the image has not been scanned
 * @since 0.10
 */
    /* * associate the specified symbol set with the image, replacing any
 * existing results.  use NULL to release the current results from the
 * image.
 * @see zbar_image_scanner_recycle_image()
 * @since 0.10
 */
    /* * image_scanner decode result iterator.
 * @returns the first decoded symbol result for an image
 * or NULL if no results are available
 */
    /* * specify the fourcc image format code for image sample data.
 * refer to the documentation for supported formats.
 * @note this does not convert the data!
 * (see zbar_image_convert() for that)
 */
    /* * associate a "sequence" (page/frame) number with this image.
 * @since 0.6
 */
    /* * specify the pixel size of the image.
 * @note this also resets the crop rectangle to the full image
 * (0, 0, width, height)
 * @note this does not affect the data!
 */
    /* * specify a rectangular region of the image to scan.
 * the rectangle will be clipped to the image boundaries.
 * defaults to the full image specified by zbar_image_set_size()
 */
    /* * specify image sample data.  when image data is no longer needed by
 * the library the specific data cleanup handler will be called
 * (unless NULL)
 * @note application image data will not be modified by the library
 */
    /* * built-in cleanup handler.
 * passes the image data buffer to free()
 */
    /* * associate user specified data value with an image.
 * @since 0.5
 */
    /* * return user specified data value associated with the image.
 * @since 0.5
 */
    /* * dump raw image data to a file for debug.
 * the data will be prefixed with a 16 byte header consisting of:
 *   - 4 bytes uint = 0x676d697a ("zimg")
 *   - 4 bytes format fourcc
 *   - 2 bytes width
 *   - 2 bytes height
 *   - 4 bytes size of following image data in bytes
 * this header can be dumped w/eg:
 * @verbatim
       od -Ax -tx1z -N16 -w4 [file]
@endverbatim
 * for some formats the image can be displayed/converted using
 * ImageMagick, eg:
 * @verbatim
       display -size 640x480+16 [-depth ?] [-sampling-factor ?x?] \
           {GRAY,RGB,UYVY,YUV}:[file]
@endverbatim
 *
 * @param image the image object to dump
 * @param filebase base filename, appended with ".XXXX.zimg" where
 * XXXX is the format fourcc
 * @returns 0 on success or a system error code on failure
 */
    /* * read back an image in the format written by zbar_image_write()
 * @note TBD
 */
    /*@}*/
    /*------------------------------------------------------------*/
/* * @name Processor interface
 * @anchor c-processor
 * high-level self-contained image processor.
 * processes video and images for barcodes, optionally displaying
 * images to a library owned output window
 */
/*@{*/
    /* * opaque standalone processor object. */
    /* * constructor.
 * if threaded is set and threading is available the processor
 * will spawn threads where appropriate to avoid blocking and
 * improve responsiveness
 */
    /* * destructor.  cleans up all resources associated with the processor
 */
    /* * (re)initialization.
 * opens a video input device and/or prepares to display output
 */
    /* * request a preferred size for the video image from the device.
 * the request may be adjusted or completely ignored by the driver.
 * @note must be called before zbar_processor_init()
 * @since 0.6
 */
    /* * request a preferred video driver interface version for
 * debug/testing.
 * @note must be called before zbar_processor_init()
 * @since 0.6
 */
    /* * request a preferred video I/O mode for debug/testing.  You will
 * get errors if the driver does not support the specified mode.
 * @verbatim
    0 = auto-detect
    1 = force I/O using read()
    2 = force memory mapped I/O using mmap()
    3 = force USERPTR I/O (v4l2 only)
@endverbatim
 * @note must be called before zbar_processor_init()
 * @since 0.7
 */
    #[no_mangle]
    fn zbar_processor_request_iomode(video: *mut zbar_processor_t,
                                     iomode: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn zbar_set_verbosity(verbosity: libc::c_int);
    #[no_mangle]
    fn zbar_increase_verbosity();
    #[no_mangle]
    fn zbar_get_symbol_name(sym: zbar_symbol_type_t) -> *const libc::c_char;
    #[no_mangle]
    fn zbar_parse_config(config_string: *const libc::c_char,
                         symbology: *mut zbar_symbol_type_t,
                         config: *mut zbar_config_t, value: *mut libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn _zbar_error_spew(object: *const libc::c_void, verbosity: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn _zbar_get_error_code(object: *const libc::c_void) -> zbar_error_t;
    #[no_mangle]
    fn zbar_symbol_get_type(symbol: *const zbar_symbol_t)
     -> zbar_symbol_type_t;
    #[no_mangle]
    fn zbar_symbol_get_data(symbol: *const zbar_symbol_t)
     -> *const libc::c_char;
    #[no_mangle]
    fn zbar_symbol_get_data_length(symbol: *const zbar_symbol_t)
     -> libc::c_uint;
    #[no_mangle]
    fn zbar_symbol_get_count(symbol: *const zbar_symbol_t) -> libc::c_int;
    #[no_mangle]
    fn zbar_symbol_next(symbol: *const zbar_symbol_t) -> *const zbar_symbol_t;
    #[no_mangle]
    fn zbar_symbol_xml(symbol: *const zbar_symbol_t,
                       buffer: *mut *mut libc::c_char,
                       buflen: *mut libc::c_uint) -> *mut libc::c_char;
    #[no_mangle]
    fn zbar_image_get_sequence(image: *const zbar_image_t) -> libc::c_uint;
    #[no_mangle]
    fn zbar_image_first_symbol(image: *const zbar_image_t)
     -> *const zbar_symbol_t;
    #[no_mangle]
    fn zbar_processor_create(threaded: libc::c_int) -> *mut zbar_processor_t;
    #[no_mangle]
    fn zbar_processor_destroy(processor: *mut zbar_processor_t);
    #[no_mangle]
    fn zbar_processor_init(processor: *mut zbar_processor_t,
                           video_device: *const libc::c_char,
                           enable_display: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn zbar_processor_request_size(processor: *mut zbar_processor_t,
                                   width: libc::c_uint, height: libc::c_uint)
     -> libc::c_int;
    #[no_mangle]
    fn zbar_processor_request_interface(processor: *mut zbar_processor_t,
                                        version: libc::c_int) -> libc::c_int;
    /* * force specific input and output formats for debug/testing.
 * @note must be called before zbar_processor_init()
 */
    #[no_mangle]
    fn zbar_processor_force_format(processor: *mut zbar_processor_t,
                                   input_format: libc::c_ulong,
                                   output_format: libc::c_ulong)
     -> libc::c_int;
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
    fn zbar_processor_set_data_handler(processor: *mut zbar_processor_t,
                                       handler:
                                           Option<zbar_image_data_handler_t>,
                                       userdata: *const libc::c_void)
     -> Option<zbar_image_data_handler_t>;
    /* * set config for indicated symbology (0 for all) to specified value.
 * @returns 0 for success, non-0 for failure (config does not apply to
 * specified symbology, or value out of range)
 * @see zbar_decoder_set_config()
 * @since 0.4
 */
    #[no_mangle]
    fn zbar_processor_set_config(processor: *mut zbar_processor_t,
                                 symbology: zbar_symbol_type_t,
                                 config: zbar_config_t, value: libc::c_int)
     -> libc::c_int;
    /* * set video control value
 * @returns 0 for success, non-0 for failure
 * @since 0.20
 * @see zbar_video_set_control()
 */
    #[no_mangle]
    fn zbar_processor_set_control(processor: *mut zbar_processor_t,
                                  control_name: *const libc::c_char,
                                  value: libc::c_int) -> libc::c_int;
    /* * get video control value
 * @returns 0 for success, non-0 for failure
 * @since 0.20
 * @see zbar_video_get_control()
 */
    #[no_mangle]
    fn zbar_processor_get_control(processor: *mut zbar_processor_t,
                                  control_name: *const libc::c_char,
                                  value: *mut libc::c_int) -> libc::c_int;
    /* * show or hide the display window owned by the library.
 * the size will be adjusted to the input size
 */
    #[no_mangle]
    fn zbar_processor_set_visible(processor: *mut zbar_processor_t,
                                  visible: libc::c_int) -> libc::c_int;
    /* * control the processor in free running video mode.
 * only works if video input is initialized. if threading is in use,
 * scanning will occur in the background, otherwise this is only
 * useful wrapping calls to zbar_processor_user_wait(). if the
 * library output window is visible, video display will be enabled.
 */
    #[no_mangle]
    fn zbar_processor_set_active(processor: *mut zbar_processor_t,
                                 active: libc::c_int) -> libc::c_int;
    /* * wait for input to the display window from the user
 * (via mouse or keyboard).
 * @returns >0 when input is received, 0 if timeout ms expired
 * with no input or -1 in case of an error
 */
    #[no_mangle]
    fn zbar_processor_user_wait(processor: *mut zbar_processor_t,
                                timeout: libc::c_int) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type zbar_symbol_type_e = libc::c_uint;
pub const ZBAR_ADDON: zbar_symbol_type_e = 1792;
pub const ZBAR_ADDON5: zbar_symbol_type_e = 1280;
pub const ZBAR_ADDON2: zbar_symbol_type_e = 512;
pub const ZBAR_SYMBOL: zbar_symbol_type_e = 255;
pub const ZBAR_CODE128: zbar_symbol_type_e = 128;
pub const ZBAR_CODE93: zbar_symbol_type_e = 93;
pub const ZBAR_QRCODE: zbar_symbol_type_e = 64;
pub const ZBAR_PDF417: zbar_symbol_type_e = 57;
pub const ZBAR_CODE39: zbar_symbol_type_e = 39;
pub const ZBAR_CODABAR: zbar_symbol_type_e = 38;
pub const ZBAR_DATABAR_EXP: zbar_symbol_type_e = 35;
pub const ZBAR_DATABAR: zbar_symbol_type_e = 34;
pub const ZBAR_I25: zbar_symbol_type_e = 25;
pub const ZBAR_COMPOSITE: zbar_symbol_type_e = 15;
pub const ZBAR_ISBN13: zbar_symbol_type_e = 14;
pub const ZBAR_EAN13: zbar_symbol_type_e = 13;
pub const ZBAR_UPCA: zbar_symbol_type_e = 12;
pub const ZBAR_ISBN10: zbar_symbol_type_e = 10;
pub const ZBAR_UPCE: zbar_symbol_type_e = 9;
pub const ZBAR_EAN8: zbar_symbol_type_e = 8;
pub const ZBAR_EAN5: zbar_symbol_type_e = 5;
pub const ZBAR_EAN2: zbar_symbol_type_e = 2;
pub const ZBAR_PARTIAL: zbar_symbol_type_e = 1;
pub const ZBAR_NONE: zbar_symbol_type_e = 0;
pub type zbar_symbol_type_t = zbar_symbol_type_e;
pub type zbar_error_e = libc::c_uint;
pub const ZBAR_ERR_NUM: zbar_error_e = 12;
pub const ZBAR_ERR_WINAPI: zbar_error_e = 11;
pub const ZBAR_ERR_CLOSED: zbar_error_e = 10;
pub const ZBAR_ERR_XPROTO: zbar_error_e = 9;
pub const ZBAR_ERR_XDISPLAY: zbar_error_e = 8;
pub const ZBAR_ERR_BUSY: zbar_error_e = 7;
pub const ZBAR_ERR_LOCKING: zbar_error_e = 6;
pub const ZBAR_ERR_SYSTEM: zbar_error_e = 5;
pub const ZBAR_ERR_INVALID: zbar_error_e = 4;
pub const ZBAR_ERR_UNSUPPORTED: zbar_error_e = 3;
pub const ZBAR_ERR_INTERNAL: zbar_error_e = 2;
pub const ZBAR_ERR_NOMEM: zbar_error_e = 1;
pub const ZBAR_OK: zbar_error_e = 0;
pub type zbar_error_t = zbar_error_e;
pub type zbar_config_e = libc::c_uint;
pub const ZBAR_CFG_Y_DENSITY: zbar_config_e = 257;
pub const ZBAR_CFG_X_DENSITY: zbar_config_e = 256;
pub const ZBAR_CFG_POSITION: zbar_config_e = 128;
pub const ZBAR_CFG_UNCERTAINTY: zbar_config_e = 64;
pub const ZBAR_CFG_MAX_LEN: zbar_config_e = 33;
pub const ZBAR_CFG_MIN_LEN: zbar_config_e = 32;
pub const ZBAR_CFG_NUM: zbar_config_e = 4;
pub const ZBAR_CFG_ASCII: zbar_config_e = 3;
pub const ZBAR_CFG_EMIT_CHECK: zbar_config_e = 2;
pub const ZBAR_CFG_ADD_CHECK: zbar_config_e = 1;
pub const ZBAR_CFG_ENABLE: zbar_config_e = 0;
pub type zbar_config_t = zbar_config_e;
pub type zbar_symbol_t = zbar_symbol_s;
pub type zbar_image_t = zbar_image_s;
pub type zbar_image_data_handler_t
    =
    unsafe extern "C" fn(_: *mut zbar_image_t, _: *const libc::c_void) -> ();
pub type zbar_processor_t = zbar_processor_s;
pub type C2RustUnnamed = libc::c_uint;
pub const XML: C2RustUnnamed = 2;
pub const RAW: C2RustUnnamed = 1;
pub const DEFAULT: C2RustUnnamed = 0;
/* * parse configuration string using zbar_parse_config()
 * and apply to processor using zbar_processor_set_config().
 * @returns 0 for success, non-0 for failure
 * @see zbar_parse_config()
 * @see zbar_processor_set_config()
 * @since 0.4
 */
#[inline]
unsafe extern "C" fn zbar_processor_parse_config(mut processor:
                                                     *mut zbar_processor_t,
                                                 mut config_string:
                                                     *const libc::c_char)
 -> libc::c_int {
    let mut sym: zbar_symbol_type_t = ZBAR_NONE;
    let mut cfg: zbar_config_t = ZBAR_CFG_ENABLE;
    let mut val: libc::c_int = 0;
    return (zbar_parse_config(config_string, &mut sym, &mut cfg, &mut val) !=
                0 || zbar_processor_set_config(processor, sym, cfg, val) != 0)
               as libc::c_int;
}
/* * display detail for last processor error to stderr.
 * @returns a non-zero value suitable for passing to exit()
 */
#[inline]
unsafe extern "C" fn zbar_processor_error_spew(mut processor:
                                                   *const zbar_processor_t,
                                               mut verbosity: libc::c_int)
 -> libc::c_int {
    return _zbar_error_spew(processor as *const libc::c_void, verbosity);
}
/* * retrieve the type code for the last processor error. */
#[inline]
unsafe extern "C" fn zbar_processor_get_error_code(mut processor:
                                                       *const zbar_processor_t)
 -> zbar_error_t {
    return _zbar_get_error_code(processor as *const libc::c_void);
}
static mut note_usage: *const libc::c_char =
    b"usage: zbarcam [options] [/dev/video?]\n\nscan and decode bar codes from a video stream\n\noptions:\n    -h, --help      display this help text\n    --version       display version information and exit\n    -q, --quiet     disable beep when symbol is decoded\n    -v, --verbose   increase debug output level\n    --verbose=N     set specific debug output level\n    --xml           use XML output format\n    --raw           output decoded symbol data without symbology prefix\n    --nodisplay     disable video display window\n    --prescale=<W>x<H>\n                    request alternate video image size from driver\n    -S<CONFIG>[=<VALUE>], --set <CONFIG>[=<VALUE>]\n                    set decoder/scanner <CONFIG> to <VALUE> (or 1)\n\n\x00"
        as *const u8 as *const libc::c_char;
static mut xml_head: *const libc::c_char =
    b"<barcodes xmlns=\'http://zbar.sourceforge.net/2008/barcode\'><source device=\'%s\'>\n\x00"
        as *const u8 as *const libc::c_char;
static mut xml_foot: *const libc::c_char =
    b"</source></barcodes>\n\x00" as *const u8 as *const libc::c_char;
static mut proc_0: *mut zbar_processor_t =
    0 as *const zbar_processor_t as *mut zbar_processor_t;
static mut quiet: libc::c_int = 0 as libc::c_int;
static mut format: C2RustUnnamed = DEFAULT;
static mut xml_buf: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
static mut xml_len: libc::c_uint = 0 as libc::c_int as libc::c_uint;
unsafe extern "C" fn usage(mut rc: libc::c_int) -> libc::c_int {
    let mut out: *mut FILE = if rc != 0 { stderr } else { stdout };
    fprintf(out, b"%s\x00" as *const u8 as *const libc::c_char, note_usage);
    return rc;
}
#[inline]
unsafe extern "C" fn parse_config(mut cfgstr: *const libc::c_char,
                                  mut i: libc::c_int, mut n: libc::c_int,
                                  mut arg: *mut libc::c_char) -> libc::c_int {
    if i >= n || *cfgstr == 0 {
        fprintf(stderr,
                b"ERROR: need argument for option: %s\n\x00" as *const u8 as
                    *const libc::c_char, arg);
        return 1 as libc::c_int
    }
    if zbar_processor_parse_config(proc_0, cfgstr) != 0 {
        fprintf(stderr,
                b"ERROR: invalid configuration setting: %s\n\x00" as *const u8
                    as *const libc::c_char, cfgstr);
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn data_handler(mut img: *mut zbar_image_t,
                                  mut userdata: *const libc::c_void) {
    let mut sym: *const zbar_symbol_t = zbar_image_first_symbol(img);
    if !sym.is_null() {
    } else {
        __assert_fail(b"sym\x00" as *const u8 as *const libc::c_char,
                      b"zbarcam/zbarcam.c\x00" as *const u8 as
                          *const libc::c_char,
                      98 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 48],
                                                &[libc::c_char; 48]>(b"void data_handler(zbar_image_t *, const void *)\x00")).as_ptr());
    }
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut current_block_6: u64;
    while !sym.is_null() {
        if !(zbar_symbol_get_count(sym) != 0) {
            let mut type_0: zbar_symbol_type_t = zbar_symbol_get_type(sym);
            if !(type_0 as libc::c_uint ==
                     ZBAR_PARTIAL as libc::c_int as libc::c_uint) {
                if format as u64 == 0 {
                    printf(b"%s:\x00" as *const u8 as *const libc::c_char,
                           zbar_get_symbol_name(type_0));
                    if fwrite(zbar_symbol_get_data(sym) as
                                  *const libc::c_void,
                              zbar_symbol_get_data_length(sym) as size_t,
                              1 as libc::c_int as size_t, stdout) !=
                           1 as libc::c_int as libc::c_ulong {
                        current_block_6 = 14155750587950065367;
                    } else { current_block_6 = 6009453772311597924; }
                } else if format as libc::c_uint ==
                              RAW as libc::c_int as libc::c_uint {
                    if fwrite(zbar_symbol_get_data(sym) as
                                  *const libc::c_void,
                              zbar_symbol_get_data_length(sym) as size_t,
                              1 as libc::c_int as size_t, stdout) !=
                           1 as libc::c_int as libc::c_ulong {
                        current_block_6 = 14155750587950065367;
                    } else { current_block_6 = 6009453772311597924; }
                } else if format as libc::c_uint ==
                              XML as libc::c_int as libc::c_uint {
                    if n == 0 {
                        printf(b"<index num=\'%u\'>\n\x00" as *const u8 as
                                   *const libc::c_char,
                               zbar_image_get_sequence(img));
                    }
                    zbar_symbol_xml(sym, &mut xml_buf, &mut xml_len);
                    if fwrite(xml_buf as *const libc::c_void,
                              xml_len as size_t, 1 as libc::c_int as size_t,
                              stdout) != 1 as libc::c_int as libc::c_ulong {
                        current_block_6 = 14155750587950065367;
                    } else { current_block_6 = 6009453772311597924; }
                } else { current_block_6 = 6009453772311597924; }
                match current_block_6 {
                    14155750587950065367 => { }
                    _ => {
                        printf(b"\n\x00" as *const u8 as *const libc::c_char);
                        n += 1
                    }
                }
            }
        }
        sym = zbar_symbol_next(sym)
    }
    if format as libc::c_uint == XML as libc::c_int as libc::c_uint && n != 0
       {
        printf(b"</index>\n\x00" as *const u8 as *const libc::c_char);
    }
    fflush(stdout);
    if quiet == 0 && n != 0 {
        fprintf(stderr, b"\x07\x00" as *const u8 as *const libc::c_char);
    };
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *const libc::c_char)
 -> libc::c_int {
    /* setup zbar library standalone processor,
     * threads will be used if available
     */
    proc_0 = zbar_processor_create(1 as libc::c_int);
    if proc_0.is_null() {
        fprintf(stderr,
                b"ERROR: unable to allocate memory?\n\x00" as *const u8 as
                    *const libc::c_char);
        return 1 as libc::c_int
    }
    zbar_processor_set_data_handler(proc_0,
                                    Some(data_handler as
                                             unsafe extern "C" fn(_:
                                                                      *mut zbar_image_t,
                                                                  _:
                                                                      *const libc::c_void)
                                                 -> ()),
                                    0 as *const libc::c_void);
    let mut video_device: *const libc::c_char =
        b"\x00" as *const u8 as *const libc::c_char;
    let mut display: libc::c_int = 1 as libc::c_int;
    let mut infmt: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut outfmt: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < argc {
        if *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize) as
               libc::c_int != '-' as i32 {
            video_device = *argv.offset(i as isize)
        } else if *(*argv.offset(i as
                                     isize)).offset(1 as libc::c_int as isize)
                      as libc::c_int != '-' as i32 {
            let mut j: libc::c_int = 0;
            j = 1 as libc::c_int;
            while *(*argv.offset(i as isize)).offset(j as isize) != 0 {
                if *(*argv.offset(i as isize)).offset(j as isize) as
                       libc::c_int == 'S' as i32 {
                    j += 1;
                    if *(*argv.offset(i as isize)).offset(j as isize) == 0 {
                        i += 1;
                        j = 0 as libc::c_int
                    }
                    if parse_config(&*(*argv.offset(i as
                                                        isize)).offset(j as
                                                                           isize),
                                    i, argc,
                                    b"-S\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char) != 0 {
                        return usage(1 as libc::c_int)
                    }
                    break ;
                } else {
                    match *(*argv.offset(i as isize)).offset(j as isize) as
                              libc::c_int {
                        104 => { return usage(0 as libc::c_int) }
                        118 => { zbar_increase_verbosity(); }
                        113 => { quiet = 1 as libc::c_int }
                        _ => {
                            fprintf(stderr,
                                    b"ERROR: unknown bundled config: -%c\n\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    *(*argv.offset(i as
                                                       isize)).offset(j as
                                                                          isize)
                                        as libc::c_int);
                            return usage(1 as libc::c_int)
                        }
                    }
                    j += 1
                }
            }
        } else if *(*argv.offset(i as
                                     isize)).offset(2 as libc::c_int as isize)
                      == 0 {
            if i < argc - 1 as libc::c_int {
                video_device =
                    *argv.offset((argc - 1 as libc::c_int) as isize)
            }
            break ;
        } else if strcmp(*argv.offset(i as isize),
                         b"--help\x00" as *const u8 as *const libc::c_char) ==
                      0 {
            return usage(0 as libc::c_int)
        } else {
            if strcmp(*argv.offset(i as isize),
                      b"--version\x00" as *const u8 as *const libc::c_char) ==
                   0 {
                return (printf(b"0.20\n\x00" as *const u8 as
                                   *const libc::c_char) <= 0 as libc::c_int)
                           as libc::c_int
            } else {
                if strcmp(*argv.offset(i as isize),
                          b"--set\x00" as *const u8 as *const libc::c_char) ==
                       0 {
                    i += 1;
                    if parse_config(*argv.offset(i as isize), i, argc,
                                    b"--set\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char) != 0 {
                        return usage(1 as libc::c_int)
                    }
                } else if strncmp(*argv.offset(i as isize),
                                  b"--set=\x00" as *const u8 as
                                      *const libc::c_char,
                                  6 as libc::c_int as libc::c_ulong) == 0 {
                    if parse_config(&*(*argv.offset(i as
                                                        isize)).offset(6 as
                                                                           libc::c_int
                                                                           as
                                                                           isize),
                                    i, argc,
                                    b"--set=\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char) != 0 {
                        return usage(1 as libc::c_int)
                    }
                } else if strcmp(*argv.offset(i as isize),
                                 b"--quiet\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                    quiet = 1 as libc::c_int
                } else if strcmp(*argv.offset(i as isize),
                                 b"--xml\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                    format = XML
                } else if strcmp(*argv.offset(i as isize),
                                 b"--raw\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                    format = RAW
                } else if strcmp(*argv.offset(i as isize),
                                 b"--nodisplay\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                    display = 0 as libc::c_int
                } else if strcmp(*argv.offset(i as isize),
                                 b"--verbose\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                    zbar_increase_verbosity();
                } else if strncmp(*argv.offset(i as isize),
                                  b"--verbose=\x00" as *const u8 as
                                      *const libc::c_char,
                                  10 as libc::c_int as libc::c_ulong) == 0 {
                    zbar_set_verbosity(strtol((*argv.offset(i as
                                                                isize)).offset(10
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize),
                                              0 as *mut *mut libc::c_char,
                                              0 as libc::c_int) as
                                           libc::c_int);
                } else if strncmp(*argv.offset(i as isize),
                                  b"--prescale=\x00" as *const u8 as
                                      *const libc::c_char,
                                  11 as libc::c_int as libc::c_ulong) == 0 {
                    let mut x: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut w: libc::c_long =
                        strtol((*argv.offset(i as
                                                 isize)).offset(11 as
                                                                    libc::c_int
                                                                    as isize),
                               &mut x, 10 as libc::c_int);
                    let mut h: libc::c_long =
                        0 as libc::c_int as libc::c_long;
                    if !x.is_null() && *x as libc::c_int == 'x' as i32 {
                        h =
                            strtol(x.offset(1 as libc::c_int as isize),
                                   0 as *mut *mut libc::c_char,
                                   10 as libc::c_int)
                    }
                    if w == 0 || h == 0 || x.is_null() ||
                           *x as libc::c_int != 'x' as i32 {
                        fprintf(stderr,
                                b"ERROR: invalid prescale: %s\n\n\x00" as
                                    *const u8 as *const libc::c_char,
                                *argv.offset(i as isize));
                        return usage(1 as libc::c_int)
                    }
                    zbar_processor_request_size(proc_0, w as libc::c_uint,
                                                h as libc::c_uint);
                } else if strncmp(*argv.offset(i as isize),
                                  b"--v4l=\x00" as *const u8 as
                                      *const libc::c_char,
                                  6 as libc::c_int as libc::c_ulong) == 0 {
                    let mut v: libc::c_long =
                        strtol((*argv.offset(i as
                                                 isize)).offset(6 as
                                                                    libc::c_int
                                                                    as isize),
                               0 as *mut *mut libc::c_char, 0 as libc::c_int);
                    zbar_processor_request_interface(proc_0,
                                                     v as libc::c_int);
                } else if strncmp(*argv.offset(i as isize),
                                  b"--iomode=\x00" as *const u8 as
                                      *const libc::c_char,
                                  9 as libc::c_int as libc::c_ulong) == 0 {
                    let mut v_0: libc::c_long =
                        strtol((*argv.offset(i as
                                                 isize)).offset(9 as
                                                                    libc::c_int
                                                                    as isize),
                               0 as *mut *mut libc::c_char, 0 as libc::c_int);
                    zbar_processor_request_iomode(proc_0, v_0 as libc::c_int);
                } else if strncmp(*argv.offset(i as isize),
                                  b"--infmt=\x00" as *const u8 as
                                      *const libc::c_char,
                                  8 as libc::c_int as libc::c_ulong) == 0 &&
                              strlen(*argv.offset(i as isize)) ==
                                  12 as libc::c_int as libc::c_ulong {
                    infmt =
                        (*(*argv.offset(i as
                                            isize)).offset(8 as libc::c_int as
                                                               isize) as
                             libc::c_int |
                             (*(*argv.offset(i as
                                                 isize)).offset(9 as
                                                                    libc::c_int
                                                                    as isize)
                                  as libc::c_int) << 8 as libc::c_int |
                             (*(*argv.offset(i as
                                                 isize)).offset(10 as
                                                                    libc::c_int
                                                                    as isize)
                                  as libc::c_int) << 16 as libc::c_int |
                             (*(*argv.offset(i as
                                                 isize)).offset(11 as
                                                                    libc::c_int
                                                                    as isize)
                                  as libc::c_int) << 24 as libc::c_int) as
                            libc::c_ulong
                } else if strncmp(*argv.offset(i as isize),
                                  b"--outfmt=\x00" as *const u8 as
                                      *const libc::c_char,
                                  9 as libc::c_int as libc::c_ulong) == 0 &&
                              strlen(*argv.offset(i as isize)) ==
                                  13 as libc::c_int as libc::c_ulong {
                    outfmt =
                        (*(*argv.offset(i as
                                            isize)).offset(9 as libc::c_int as
                                                               isize) as
                             libc::c_int |
                             (*(*argv.offset(i as
                                                 isize)).offset(10 as
                                                                    libc::c_int
                                                                    as isize)
                                  as libc::c_int) << 8 as libc::c_int |
                             (*(*argv.offset(i as
                                                 isize)).offset(11 as
                                                                    libc::c_int
                                                                    as isize)
                                  as libc::c_int) << 16 as libc::c_int |
                             (*(*argv.offset(i as
                                                 isize)).offset(12 as
                                                                    libc::c_int
                                                                    as isize)
                                  as libc::c_int) << 24 as libc::c_int) as
                            libc::c_ulong
                } else {
                    fprintf(stderr,
                            b"ERROR: unknown option argument: %s\n\n\x00" as
                                *const u8 as *const libc::c_char,
                            *argv.offset(i as isize));
                    return usage(1 as libc::c_int)
                }
            }
        }
        i += 1
    }
    if infmt != 0 || outfmt != 0 {
        zbar_processor_force_format(proc_0, infmt, outfmt);
    }
    /* open video device, open window */
    if zbar_processor_init(proc_0, video_device, display) != 0 ||
           display != 0 &&
               zbar_processor_set_visible(proc_0, 1 as libc::c_int) != 0 {
        return zbar_processor_error_spew(proc_0, 0 as libc::c_int)
    }
    if format as libc::c_uint == XML as libc::c_int as libc::c_uint {
        printf(xml_head, video_device);
        fflush(stdout);
    }
    /* start video */
    let mut active: libc::c_int = 1 as libc::c_int;
    if zbar_processor_set_active(proc_0, active) != 0 {
        return zbar_processor_error_spew(proc_0, 0 as libc::c_int)
    }
    /* let the callback handle data */
    let mut rc: libc::c_int = 0;
    loop  {
        rc = zbar_processor_user_wait(proc_0, -(1 as libc::c_int));
        if !(rc >= 0 as libc::c_int) { break ; }
        if rc == 'q' as i32 || rc == 'Q' as i32 { break ; }
        // HACK: controls are known on V4L2 by ID, not by name. This is also
        // not compatible with other platforms
        if rc == 'b' as i32 || rc == 'B' as i32 {
            let mut value: libc::c_int = 0;
            zbar_processor_get_control(proc_0,
                                       b"Brightness\x00" as *const u8 as
                                           *const libc::c_char, &mut value);
            value += 1;
            zbar_processor_set_control(proc_0,
                                       b"Brightness\x00" as *const u8 as
                                           *const libc::c_char, value);
        }
        if rc == 'n' as i32 || rc == 'N' as i32 {
            let mut value_0: libc::c_int = 0;
            zbar_processor_get_control(proc_0,
                                       b"Brightness\x00" as *const u8 as
                                           *const libc::c_char, &mut value_0);
            value_0 -= 1;
            zbar_processor_set_control(proc_0,
                                       b"Brightness\x00" as *const u8 as
                                           *const libc::c_char, value_0);
        }
        if rc == ' ' as i32 {
            active = (active == 0) as libc::c_int;
            if zbar_processor_set_active(proc_0, active) != 0 {
                return zbar_processor_error_spew(proc_0, 0 as libc::c_int)
            }
        }
    }
    /* report any errors that aren't "window closed" */
    if rc != 0 && rc != 'q' as i32 && rc != 'Q' as i32 &&
           zbar_processor_get_error_code(proc_0) as libc::c_uint !=
               ZBAR_ERR_CLOSED as libc::c_int as libc::c_uint {
        return zbar_processor_error_spew(proc_0, 0 as libc::c_int)
    }
    /* free resources (leak check) */
    zbar_processor_destroy(proc_0);
    if format as libc::c_uint == XML as libc::c_int as libc::c_uint {
        printf(b"%s\x00" as *const u8 as *const libc::c_char, xml_foot);
        fflush(stdout);
    }
    return 0 as libc::c_int;
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *const libc::c_char) as i32)
    }
}
