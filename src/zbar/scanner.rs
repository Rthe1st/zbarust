use ::libc;
extern {
    pub type zbar_decoder_s;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
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
     *------------------------------------------------------------------------ */
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
    /* @{ */
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
    /* @} */
    /* ------------------------------------------------------------ */
    /* * @name Symbol interface
     * decoded barcode symbol result object.  stores type, data, and image
     * location of decoded symbol.  all memory is owned by the library
     */
    /* @{ */
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
    /* @} */
    /* ------------------------------------------------------------ */
    /* * @name Symbol Set interface
     * container for decoded result symbols associated with an image
     * or a composite symbol.
     * @since 0.10
     */
    /* @{ */
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
    /* @} */
    /* ------------------------------------------------------------ */
    /* * @name Image interface
     * stores image data samples along with associated format and size
     * metadata
     */
    /* @{ */
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
    /* @} */
    /* ------------------------------------------------------------ */
    /* * @name Processor interface
     * @anchor c-processor
     * high-level self-contained image processor.
     * processes video and images for barcodes, optionally displaying
     * images to a library owned output window
     */
    /* @{ */
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
    /* * force specific input and output formats for debug/testing.
     * @note must be called before zbar_processor_init()
     */
    /* * setup result handler callback.
     * the specified function will be called by the processor whenever
     * new results are available from the video stream or a static image.
     * pass a NULL value to disable callbacks.
     * @param processor the object on which to set the handler.
     * @param handler the function to call when new results are available.
     * @param userdata is set as with zbar_processor_set_userdata().
     * @returns the previously registered handler
     */
    /* * associate user specified data value with the processor.
     * @since 0.6
     */
    /* * return user specified data value associated with the processor.
     * @since 0.6
     */
    /* * set config for indicated symbology (0 for all) to specified value.
     * @returns 0 for success, non-0 for failure (config does not apply to
     * specified symbology, or value out of range)
     * @see zbar_decoder_set_config()
     * @since 0.4
     */
    /* * set video control value
     * @returns 0 for success, non-0 for failure
     * @since 0.20
     * @see zbar_video_set_control()
     */
    /* * get video control value
     * @returns 0 for success, non-0 for failure
     * @since 0.20
     * @see zbar_video_get_control()
     */
    /* * parse configuration string using zbar_parse_config()
     * and apply to processor using zbar_processor_set_config().
     * @returns 0 for success, non-0 for failure
     * @see zbar_parse_config()
     * @see zbar_processor_set_config()
     * @since 0.4
     */
    /* * retrieve the current state of the ouput window.
     * @returns 1 if the output window is currently displayed, 0 if not.
     * @returns -1 if an error occurs
     */
    /* * show or hide the display window owned by the library.
     * the size will be adjusted to the input size
     */
    /* * control the processor in free running video mode.
     * only works if video input is initialized. if threading is in use,
     * scanning will occur in the background, otherwise this is only
     * useful wrapping calls to zbar_processor_user_wait(). if the
     * library output window is visible, video display will be enabled.
     */
    /* * retrieve decode results for last scanned image/frame.
     * @returns the symbol set result container or NULL if no results are
     * available
     * @note the returned symbol set has its reference count incremented;
     * ensure that the count is decremented after use
     * @since 0.10
     */
    /* * wait for input to the display window from the user
     * (via mouse or keyboard).
     * @returns >0 when input is received, 0 if timeout ms expired
     * with no input or -1 in case of an error
     */
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
    /* * process the provided image for barcodes.
     * if the library window is visible, the image will be displayed.
     * @returns >0 if symbols were successfully decoded,
     * 0 if no symbols were found or -1 if an error occurs
     */
    /* * display detail for last processor error to stderr.
     * @returns a non-zero value suitable for passing to exit()
     */
    /* * retrieve the detail string for the last processor error. */
    /* * retrieve the type code for the last processor error. */
    /* @} */
    /* ------------------------------------------------------------ */
    /* * @name Video interface
     * @anchor c-video
     * mid-level video source abstraction.
     * captures images from a video device
     */
    /* @{ */
    /* * opaque video object. */
    /* * constructor. */
    /* * destructor. */
    /* * open and probe a video device.
     * the device specified by platform specific unique name
     * (v4l device node path in *nix eg "/dev/video",
     *  DirectShow DevicePath property in windows).
     * @returns 0 if successful or -1 if an error occurs
     */
    /* * retrieve file descriptor associated with open *nix video device
     * useful for using select()/poll() to tell when new images are
     * available (NB v4l2 only!!).
     * @returns the file descriptor or -1 if the video device is not open
     * or the driver only supports v4l1
     */
    /* * request a preferred size for the video image from the device.
     * the request may be adjusted or completely ignored by the driver.
     * @returns 0 if successful or -1 if the video device is already
     * initialized
     * @since 0.6
     */
    /* * request a preferred driver interface version for debug/testing.
     * @note must be called before zbar_video_open()
     * @since 0.6
     */
    /* * request a preferred I/O mode for debug/testing.  You will get
     * errors if the driver does not support the specified mode.
     * @verbatim
        0 = auto-detect
        1 = force I/O using read()
        2 = force memory mapped I/O using mmap()
        3 = force USERPTR I/O (v4l2 only)
    @endverbatim
     * @note must be called before zbar_video_open()
     * @since 0.7
     */
    /* * retrieve current output image width.
     * @returns the width or 0 if the video device is not open
     */
    /* * retrieve current output image height.
     * @returns the height or 0 if the video device is not open
     */
    /* * initialize video using a specific format for debug.
     * use zbar_negotiate_format() to automatically select and initialize
     * the best available format
     */
    /* * start/stop video capture.
     * all buffered images are retired when capture is disabled.
     * @returns 0 if successful or -1 if an error occurs
     */
    /* * retrieve next captured image.  blocks until an image is available.
     * @returns NULL if video is not enabled or an error occurs
     */
    /* * set video control value (integer).
     * @returns 0 for success, non-0 for failure
     * @since 0.20
     * @see zbar_processor_set_control()
     */
    /* * get video control value (integer).
     * @returns 0 for success, non-0 for failure
     * @since 0.20
     * @see zbar_processor_get_control()
     */
    /* * get available controls from video source
     * @returns 0 for success, non-0 for failure
     * @since 0.20
     */
    /* * display detail for last video error to stderr.
     * @returns a non-zero value suitable for passing to exit()
     */
    /* * retrieve the detail string for the last video error. */
    /* * retrieve the type code for the last video error. */
    /* @} */
    /* ------------------------------------------------------------ */
    /* * @name Window interface
     * @anchor c-window
     * mid-level output window abstraction.
     * displays images to user-specified platform specific output window
     */
    /* @{ */
    /* * opaque window object. */
    /* * constructor. */
    /* * destructor. */
    /* * associate reader with an existing platform window.
     * This can be any "Drawable" for X Windows or a "HWND" for windows.
     * input images will be scaled into the output window.
     * pass NULL to detach from the resource, further input will be
     * ignored
     */
    /* * control content level of the reader overlay.
     * the overlay displays graphical data for informational or debug
     * purposes.  higher values increase the level of annotation (possibly
     * decreasing performance). @verbatim
        0 = disable overlay
        1 = outline decoded symbols (default)
        2 = also track and display input frame rate
    @endverbatim
     */
    /* * retrieve current content level of reader overlay.
     * @see zbar_window_set_overlay()
     * @since 0.10
     */
    /* * draw a new image into the output window. */
    /* * redraw the last image (exposure handler). */
    /* * resize the image window (reconfigure handler).
     * this does @em not update the contents of the window
     * @since 0.3, changed in 0.4 to not redraw window
     */
    /* * display detail for last window error to stderr.
     * @returns a non-zero value suitable for passing to exit()
     */
    /* * retrieve the detail string for the last window error. */
    /* * retrieve the type code for the last window error. */
    /* * select a compatible format between video input and output window.
     * the selection algorithm attempts to use a format shared by
     * video input and window output which is also most useful for
     * barcode scanning.  if a format conversion is necessary, it will
     * heuristically attempt to minimize the cost of the conversion
     */
    /* @} */
    /* ------------------------------------------------------------ */
    /* * @name Image Scanner interface
     * @anchor c-imagescanner
     * mid-level image scanner interface.
     * reads barcodes from 2-D images
     */
    /* @{ */
    /* * opaque image scanner object. */
    /* * constructor. */
    /* * destructor. */
    /* * setup result handler callback.
     * the specified function will be called by the scanner whenever
     * new results are available from a decoded image.
     * pass a NULL value to disable callbacks.
     * @returns the previously registered handler
     */
    /* * set config for indicated symbology (0 for all) to specified value.
     * @returns 0 for success, non-0 for failure (config does not apply to
     * specified symbology, or value out of range)
     * @see zbar_decoder_set_config()
     * @since 0.4
     */
    /* * parse configuration string using zbar_parse_config()
     * and apply to image scanner using zbar_image_scanner_set_config().
     * @returns 0 for success, non-0 for failure
     * @see zbar_parse_config()
     * @see zbar_image_scanner_set_config()
     * @since 0.4
     */
    /* * enable or disable the inter-image result cache (default disabled).
     * mostly useful for scanning video frames, the cache filters
     * duplicate results from consecutive images, while adding some
     * consistency checking and hysteresis to the results.
     * this interface also clears the cache
     */
    /* * remove any previously decoded results from the image scanner and the
     * specified image.  somewhat more efficient version of
     * zbar_image_set_symbols(image, NULL) which may retain memory for
     * subsequent decodes
     * @since 0.10
     */
    /* * retrieve decode results for last scanned image.
     * @returns the symbol set result container or NULL if no results are
     * available
     * @note the symbol set does not have its reference count adjusted;
     * ensure that the count is incremented if the results may be kept
     * after the next image is scanned
     * @since 0.10
     */
    /* * scan for symbols in provided image.  The image format must be
     * "Y800" or "GRAY".
     * @returns >0 if symbols were successfully decoded from the image,
     * 0 if no symbols were found or -1 if an error occurs
     * @see zbar_image_convert()
     * @since 0.9 - changed to only accept grayscale images
     */
    /* @} */
    /* ------------------------------------------------------------ */
    /* * @name Decoder interface
     * @anchor c-decoder
     * low-level bar width stream decoder interface.
     * identifies symbols and extracts encoded data
     */
    /* @{ */
    /* * opaque decoder object. */
    /* * decoder data handler callback function.
     * called by decoder when new data has just been decoded
     */
    /* * constructor. */
    /* * destructor. */
    /* * set config for indicated symbology (0 for all) to specified value.
     * @returns 0 for success, non-0 for failure (config does not apply to
     * specified symbology, or value out of range)
     * @since 0.4
     */
    /* * parse configuration string using zbar_parse_config()
     * and apply to decoder using zbar_decoder_set_config().
     * @returns 0 for success, non-0 for failure
     * @see zbar_parse_config()
     * @see zbar_decoder_set_config()
     * @since 0.4
     */
    /* * retrieve symbology boolean config settings.
     * @returns a bitmask indicating which configs are currently set for the
     * specified symbology.
     * @since 0.11
     */
    /* * clear all decoder state.
     * any partial symbols are flushed
     */
    /* * mark start of a new scan pass.
     * clears any intra-symbol state and resets color to ::ZBAR_SPACE.
     * any partially decoded symbol state is retained
     */
    #[no_mangle]
    fn zbar_decoder_new_scan(decoder: *mut zbar_decoder_t);
    #[no_mangle]
    fn zbar_decoder_reset(decoder: *mut zbar_decoder_t);
    #[no_mangle]
    fn zbar_decode_width(decoder: *mut zbar_decoder_t, width: libc::c_uint) -> zbar_symbol_type_t;
}
pub type zbar_color_e = libc::c_uint;
pub const ZBAR_BAR: zbar_color_e = 1;
pub const ZBAR_SPACE: zbar_color_e = 0;
pub type zbar_color_t = zbar_color_e;
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
pub type zbar_decoder_t = zbar_decoder_s;
/* scanner state */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zbar_scanner_s {
    pub decoder: *mut zbar_decoder_t,
    pub y1_min_thresh: libc::c_uint,
    pub x: libc::c_uint,
    pub y0: [libc::c_int; 4],
    pub y1_sign: libc::c_int,
    pub y1_thresh: libc::c_uint,
    pub cur_edge: libc::c_uint,
    pub last_edge: libc::c_uint,
    pub width: libc::c_uint,
}
pub type zbar_scanner_t = zbar_scanner_s;
/* last element width */
#[no_mangle]
pub unsafe extern fn zbar_scanner_create(mut dcode: *mut zbar_decoder_t) -> *mut zbar_scanner_t {
    let mut scn: *mut zbar_scanner_t =
        malloc(::std::mem::size_of::<zbar_scanner_t>() as libc::c_ulong) as *mut zbar_scanner_t;
    (*scn).decoder = dcode;
    (*scn).y1_min_thresh = 4 as libc::c_int as libc::c_uint;
    zbar_scanner_reset(scn);
    return scn;
}
#[no_mangle]
pub unsafe extern fn zbar_scanner_destroy(mut scn: *mut zbar_scanner_t) {
    free(scn as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern fn zbar_scanner_reset(mut scn: *mut zbar_scanner_t) -> zbar_symbol_type_t {
    memset(
        &mut (*scn).x as *mut libc::c_uint as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<zbar_scanner_t>() as libc::c_ulong)
            .wrapping_sub(12 as libc::c_ulong),
    );
    (*scn).y1_thresh = (*scn).y1_min_thresh;
    if !(*scn).decoder.is_null() {
        zbar_decoder_reset((*scn).decoder);
    }
    return ZBAR_NONE;
}
/* * process next bar/space width from input stream.
 * the width is in arbitrary relative units.  first value of a scan
 * is ::ZBAR_SPACE width, alternating from there.
 * @returns appropriate symbol type if width completes
 * decode of a symbol (data is available for retrieval)
 * @returns ::ZBAR_PARTIAL as a hint if part of a symbol was decoded
 * @returns ::ZBAR_NONE (0) if no new symbol data is available
 */
/* * retrieve color of @em next element passed to
 * zbar_decode_width(). */
/* * retrieve last decoded data.
 * @returns the data string or NULL if no new data available.
 * the returned data buffer is owned by library, contents are only
 * valid between non-0 return from zbar_decode_width and next library
 * call
 */
/* * retrieve length of binary data.
 * @returns the length of the decoded data or 0 if no new data
 * available.
 */
/* * retrieve last decoded symbol type.
 * @returns the type or ::ZBAR_NONE if no new data available
 */
/* * retrieve modifier flags for the last decoded symbol.
 * @returns a bitmask indicating which characteristics were detected
 * during decoding.
 * @since 0.11
 */
/* * retrieve last decode direction.
 * @returns 1 for forward and -1 for reverse
 * @returns 0 if the decode direction is unknown or does not apply
 * @since 0.11
 */
/* * setup data handler callback.
 * the registered function will be called by the decoder
 * just before zbar_decode_width() returns a non-zero value.
 * pass a NULL value to disable callbacks.
 * @returns the previously registered handler
 */
/* * associate user specified data value with the decoder. */
/* * return user specified data value associated with the decoder. */
/* @} */
/* ------------------------------------------------------------ */
/* * @name Scanner interface
 * @anchor c-scanner
 * low-level linear intensity sample stream scanner interface.
 * identifies "bar" edges and measures width between them.
 * optionally passes to bar width decoder
 */
/* @{ */
/* * opaque scanner object. */
/* * constructor.
 * if decoder is non-NULL it will be attached to scanner
 * and called automatically at each new edge
 * current color is initialized to ::ZBAR_SPACE
 * (so an initial BAR->SPACE transition may be discarded)
 */
/* * destructor. */
/* * clear all scanner state.
 * also resets an associated decoder
 */
/* * mark start of a new scan pass. resets color to ::ZBAR_SPACE.
 * also updates an associated decoder.
 * @returns any decode results flushed from the pipeline
 * @note when not using callback handlers, the return value should
 * be checked the same as zbar_scan_y()
 * @note call zbar_scanner_flush() at least twice before calling this
 * method to ensure no decode results are lost
 */
/* * flush scanner processing pipeline.
 * forces current scanner position to be a scan boundary.
 * call multiple times (max 3) to completely flush decoder.
 * @returns any decode/scan results flushed from the pipeline
 * @note when not using callback handlers, the return value should
 * be checked the same as zbar_scan_y()
 * @since 0.9
 */
/* * process next sample intensity value.
 * intensity (y) is in arbitrary relative units.
 * @returns result of zbar_decode_width() if a decoder is attached,
 * otherwise @returns (::ZBAR_PARTIAL) when new edge is detected
 * or 0 (::ZBAR_NONE) if no new edge is detected
 */
/* * process next sample from RGB (or BGR) triple. */
/* * retrieve last scanned width. */
#[no_mangle]
pub unsafe extern fn zbar_scanner_get_width(mut scn: *const zbar_scanner_t) -> libc::c_uint {
    return (*scn).width;
}
/* * retrieve sample position of last edge.
 * @since 0.10
 */
#[no_mangle]
pub unsafe extern fn zbar_scanner_get_edge(
    mut scn: *const zbar_scanner_t,
    mut offset: libc::c_uint,
    mut prec: libc::c_int,
) -> libc::c_uint {
    let mut edge: libc::c_uint = (*scn)
        .last_edge
        .wrapping_sub(offset)
        .wrapping_sub(((1 as libc::c_int) << 5 as libc::c_int) as libc::c_uint)
        .wrapping_sub(((1 as libc::c_int) << 5 as libc::c_int - 1 as libc::c_int) as libc::c_uint);
    prec = 5 as libc::c_int - prec;
    if prec > 0 as libc::c_int {
        return edge >> prec;
    } else if prec == 0 {
        return edge;
    } else {
        return edge << -prec;
    };
}
/* * retrieve last scanned color. */
#[no_mangle]
pub unsafe extern fn zbar_scanner_get_color(mut scn: *const zbar_scanner_t) -> zbar_color_t {
    return if (*scn).y1_sign <= 0 as libc::c_int {
        ZBAR_SPACE as libc::c_int
    } else {
        ZBAR_BAR as libc::c_int
    } as zbar_color_t;
}
#[inline]
unsafe extern fn calc_thresh(mut scn: *mut zbar_scanner_t) -> libc::c_uint {
    /* threshold 1st to improve noise rejection */
    let mut dx: libc::c_uint = 0;
    let mut thresh: libc::c_uint = (*scn).y1_thresh;
    let mut t: libc::c_ulong = 0;
    if thresh <= (*scn).y1_min_thresh || (*scn).width == 0 {
        return (*scn).y1_min_thresh;
    }
    /* slowly return threshold to min */
    dx = ((*scn).x << 5 as libc::c_int).wrapping_sub((*scn).last_edge);
    t = thresh.wrapping_mul(dx) as libc::c_ulong;
    t = t.wrapping_div((*scn).width as libc::c_ulong);
    t = t.wrapping_div(8 as libc::c_int as libc::c_ulong);
    if thresh as libc::c_ulong > t {
        thresh = (thresh as libc::c_ulong).wrapping_sub(t) as libc::c_uint as libc::c_uint;
        if thresh > (*scn).y1_min_thresh {
            return thresh;
        }
    }
    (*scn).y1_thresh = (*scn).y1_min_thresh;
    return (*scn).y1_min_thresh;
}
#[inline]
unsafe extern fn process_edge(
    mut scn: *mut zbar_scanner_t,
    mut y1: libc::c_int,
) -> zbar_symbol_type_t {
    if (*scn).y1_sign == 0 {
        (*scn).cur_edge = (((1 as libc::c_int) << 5 as libc::c_int)
            + ((1 as libc::c_int) << 5 as libc::c_int - 1 as libc::c_int))
            as libc::c_uint;
        (*scn).last_edge = (*scn).cur_edge
    } else if (*scn).last_edge == 0 {
        (*scn).last_edge = (*scn).cur_edge
    }
    (*scn).width = (*scn).cur_edge.wrapping_sub((*scn).last_edge);
    (*scn).last_edge = (*scn).cur_edge;
    /* pass to decoder */
    if !(*scn).decoder.is_null() {
        return zbar_decode_width((*scn).decoder, (*scn).width);
    }
    return ZBAR_PARTIAL;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern fn zbar_scanner_flush(mut scn: *mut zbar_scanner_t) -> zbar_symbol_type_t {
    let mut x: libc::c_uint = 0;
    if (*scn).y1_sign == 0 {
        return ZBAR_NONE;
    }
    x = ((*scn).x << 5 as libc::c_int)
        .wrapping_add(((1 as libc::c_int) << 5 as libc::c_int - 1 as libc::c_int) as libc::c_uint);
    if (*scn).cur_edge != x || (*scn).y1_sign > 0 as libc::c_int {
        let mut edge: zbar_symbol_type_t = process_edge(scn, -(*scn).y1_sign);
        (*scn).cur_edge = x;
        (*scn).y1_sign = -(*scn).y1_sign;
        return edge;
    }
    (*scn).width = 0 as libc::c_int as libc::c_uint;
    (*scn).y1_sign = (*scn).width as libc::c_int;
    if !(*scn).decoder.is_null() {
        return zbar_decode_width((*scn).decoder, 0 as libc::c_int as libc::c_uint);
    }
    return ZBAR_PARTIAL;
}
#[no_mangle]
pub unsafe extern fn zbar_scanner_new_scan(mut scn: *mut zbar_scanner_t) -> zbar_symbol_type_t {
    let mut edge: zbar_symbol_type_t = ZBAR_NONE;
    while (*scn).y1_sign != 0 {
        let mut tmp: zbar_symbol_type_t = zbar_scanner_flush(scn);
        if (tmp as libc::c_uint) < 0 as libc::c_int as libc::c_uint
            || tmp as libc::c_uint > edge as libc::c_uint
        {
            edge = tmp
        }
    }
    /* reset scanner and associated decoder */
    memset(
        &mut (*scn).x as *mut libc::c_uint as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<zbar_scanner_t>() as libc::c_ulong)
            .wrapping_sub(12 as libc::c_ulong),
    );
    (*scn).y1_thresh = (*scn).y1_min_thresh;
    if !(*scn).decoder.is_null() {
        zbar_decoder_new_scan((*scn).decoder);
    }
    return edge;
}
#[no_mangle]
pub unsafe extern fn zbar_scan_y(
    mut scn: *mut zbar_scanner_t,
    mut y: libc::c_int,
) -> zbar_symbol_type_t {
    /* FIXME calc and clip to max y range... */
    /* retrieve short value history */
    let mut x: libc::c_int = (*scn).x as libc::c_int;
    let mut y0_1: libc::c_int = (*scn).y0[(x - 1 as libc::c_int & 3 as libc::c_int) as usize];
    let mut y0_0: libc::c_int = y0_1;
    let mut y0_2: libc::c_int = 0;
    let mut y0_3: libc::c_int = 0;
    let mut y1_1: libc::c_int = 0;
    let mut y2_1: libc::c_int = 0;
    let mut y2_2: libc::c_int = 0;
    let mut edge: zbar_symbol_type_t = ZBAR_NONE;
    if x != 0 {
        /* update weighted moving average */
        y0_0 += ((y - y0_1) as libc::c_uint).wrapping_mul(
            ((0.78f64
                * ((1 as libc::c_int) << 5 as libc::c_int + 1 as libc::c_int) as libc::c_double
                + 1 as libc::c_int as libc::c_double)
                / 2 as libc::c_int as libc::c_double) as libc::c_uint,
        ) as libc::c_int
            >> 5 as libc::c_int;
        (*scn).y0[(x & 3 as libc::c_int) as usize] = y0_0
    } else {
        (*scn).y0[3 as libc::c_int as usize] = y;
        (*scn).y0[2 as libc::c_int as usize] = (*scn).y0[3 as libc::c_int as usize];
        (*scn).y0[1 as libc::c_int as usize] = (*scn).y0[2 as libc::c_int as usize];
        (*scn).y0[0 as libc::c_int as usize] = (*scn).y0[1 as libc::c_int as usize];
        y0_1 = (*scn).y0[0 as libc::c_int as usize];
        y0_0 = y0_1
    }
    y0_2 = (*scn).y0[(x - 2 as libc::c_int & 3 as libc::c_int) as usize];
    y0_3 = (*scn).y0[(x - 3 as libc::c_int & 3 as libc::c_int) as usize];
    /* 1st differential @ x-1 */
    y1_1 = y0_1 - y0_2;
    let mut y1_2: libc::c_int = y0_2 - y0_3;
    if abs(y1_1) < abs(y1_2)
        && (y1_1 >= 0 as libc::c_int) as libc::c_int == (y1_2 >= 0 as libc::c_int) as libc::c_int
    {
        y1_1 = y1_2
    }
    /* 2nd differentials @ x-1 & x-2 */
    y2_1 = y0_0 - y0_1 * 2 as libc::c_int + y0_2;
    y2_2 = y0_1 - y0_2 * 2 as libc::c_int + y0_3;
    edge = ZBAR_NONE;
    /* 2nd zero-crossing is 1st local min/max - could be edge */
    if (y2_1 == 0
        || (if y2_1 > 0 as libc::c_int {
            (y2_2 < 0 as libc::c_int) as libc::c_int
        } else {
            (y2_2 > 0 as libc::c_int) as libc::c_int
        }) != 0)
        && calc_thresh(scn) <= abs(y1_1) as libc::c_uint
    {
        /* check for 1st sign change */
        let mut y1_rev: libc::c_char = if (*scn).y1_sign > 0 as libc::c_int {
            (y1_1 < 0 as libc::c_int) as libc::c_int
        } else {
            (y1_1 > 0 as libc::c_int) as libc::c_int
        } as libc::c_char;
        if y1_rev != 0 {
            /* intensity change reversal - finalize previous edge */
            edge = process_edge(scn, y1_1)
        }
        if y1_rev as libc::c_int != 0 || abs((*scn).y1_sign) < abs(y1_1) {
            let mut d: libc::c_int = 0;
            (*scn).y1_sign = y1_1;
            /* adaptive thresholding */
            /* start at multiple of new min/max */
            (*scn).y1_thresh = (abs(y1_1) as libc::c_uint)
                .wrapping_mul(
                    ((0.44f64
                        * ((1 as libc::c_int) << 5 as libc::c_int + 1 as libc::c_int)
                            as libc::c_double
                        + 1 as libc::c_int as libc::c_double)
                        / 2 as libc::c_int as libc::c_double) as libc::c_uint,
                )
                .wrapping_add(
                    ((1 as libc::c_int) << 5 as libc::c_int - 1 as libc::c_int) as libc::c_uint,
                )
                >> 5 as libc::c_int;
            if (*scn).y1_thresh < (*scn).y1_min_thresh {
                (*scn).y1_thresh = (*scn).y1_min_thresh
            }
            /* update current edge */
            d = y2_1 - y2_2;
            (*scn).cur_edge = ((1 as libc::c_int) << 5 as libc::c_int) as libc::c_uint;
            if d == 0 {
                (*scn).cur_edge >>= 1 as libc::c_int
            } else if y2_1 != 0 {
                /* interpolate zero crossing */
                (*scn).cur_edge = (*scn).cur_edge.wrapping_sub(
                    (((y2_1 << 5 as libc::c_int) + 1 as libc::c_int) / d) as libc::c_uint,
                )
            }
            (*scn).cur_edge = (*scn).cur_edge.wrapping_add((x << 5 as libc::c_int) as libc::c_uint)
        }
    }
    /* FIXME add fall-thru pass to decoder after heuristic "idle" period
    (eg, 6-8 * last width) */
    (*scn).x = (x + 1 as libc::c_int) as libc::c_uint;
    return edge;
}
/* undocumented API for drawing cutesy debug graphics */
#[no_mangle]
pub unsafe extern fn zbar_scanner_get_state(
    mut scn: *const zbar_scanner_t,
    mut x: *mut libc::c_uint,
    mut cur_edge: *mut libc::c_uint,
    mut last_edge: *mut libc::c_uint,
    mut y0: *mut libc::c_int,
    mut y1: *mut libc::c_int,
    mut y2: *mut libc::c_int,
    mut y1_thresh: *mut libc::c_int,
) {
    let mut y0_0: libc::c_int = (*scn).y0[((*scn).x.wrapping_sub(1 as libc::c_int as libc::c_uint)
        & 3 as libc::c_int as libc::c_uint) as usize];
    let mut y0_1: libc::c_int = (*scn).y0[((*scn).x.wrapping_sub(2 as libc::c_int as libc::c_uint)
        & 3 as libc::c_int as libc::c_uint) as usize];
    let mut y0_2: libc::c_int = (*scn).y0[((*scn).x.wrapping_sub(3 as libc::c_int as libc::c_uint)
        & 3 as libc::c_int as libc::c_uint) as usize];
    let mut mut_scn: *mut zbar_scanner_t = 0 as *mut zbar_scanner_t;
    if !x.is_null() {
        *x = (*scn).x.wrapping_sub(1 as libc::c_int as libc::c_uint)
    }
    if !last_edge.is_null() {
        *last_edge = (*scn).last_edge
    }
    if !y0.is_null() {
        *y0 = y0_1
    }
    if !y1.is_null() {
        *y1 = y0_1 - y0_2
    }
    if !y2.is_null() {
        *y2 = y0_0 - y0_1 * 2 as libc::c_int + y0_2
    }
    /* NB not quite accurate (uses updated x) */
    mut_scn = scn as *mut zbar_scanner_t;
    if !y1_thresh.is_null() {
        *y1_thresh = calc_thresh(mut_scn) as libc::c_int
    };
}
