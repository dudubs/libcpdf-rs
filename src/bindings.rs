use std::os::raw::*;

pub type CpdfPermission = c_int;
pub type CpdfEncryptionMethod = c_int;
pub type CpdfPaperSize = c_int;
pub type CpdfAnchor = c_int;
pub type CpdfFont = c_int;
pub type CpdfJustification = c_int;
pub type CpdfLayout = c_int;
pub type CpdfPageMode = c_int;
pub type CpdfPageLabelStyle = c_int;

pub const CPDF_PERMISSION_NOEDIT: CpdfPermission = 0;
pub const CPDF_PERMISSION_NOPRINT: CpdfPermission = 1;
pub const CPDF_PERMISSION_NOCOPY: CpdfPermission = 2;
pub const CPDF_PERMISSION_NOANNOT: CpdfPermission = 3;
pub const CPDF_PERMISSION_NOFORMS: CpdfPermission = 4;
pub const CPDF_PERMISSION_NOEXTRACT: CpdfPermission = 5;
pub const CPDF_PERMISSION_NOASSEMBLE: CpdfPermission = 6;
pub const CPDF_PERMISSION_NOHQPRINT: CpdfPermission = 7;
pub const CPDF_ENCRYPTIONMETHOD_PDF40BIT: CpdfEncryptionMethod = 0;
pub const CPDF_ENCRYPTIONMETHOD_PDF128BIT: CpdfEncryptionMethod = 1;
pub const CPDF_ENCRYPTIONMETHOD_AES128BIT_FALSE: CpdfEncryptionMethod = 2;
pub const CPDF_ENCRYPTIONMETHOD_AES128BITTRUE: CpdfEncryptionMethod = 3;
pub const CPDF_ENCRYPTIONMETHOD_AES256BITFALSE: CpdfEncryptionMethod = 4;
pub const CPDF_ENCRYPTIONMETHOD_AES256BITTRUE: CpdfEncryptionMethod = 5;
pub const CPDF_ENCRYPTIONMETHOD_AES256BITISOFALSE: CpdfEncryptionMethod = 6;
pub const CPDF_ENCRYPTIONMETHOD_AES256BITISOTRUE: CpdfEncryptionMethod = 7;
pub const CPDF_PAPERSIZE_A0PORTRAIT: CpdfPaperSize = 0;
pub const CPDF_PAPERSIZE_A1PORTRAIT: CpdfPaperSize = 1;
pub const CPDF_PAPERSIZE_A2PORTRAIT: CpdfPaperSize = 2;
pub const CPDF_PAPERSIZE_A3PORTRAIT: CpdfPaperSize = 3;
pub const CPDF_PAPERSIZE_A4PORTRAIT: CpdfPaperSize = 4;
pub const CPDF_PAPERSIZE_A5PORTRAIT: CpdfPaperSize = 5;
pub const CPDF_PAPERSIZE_A0LANDSCAPE: CpdfPaperSize = 6;
pub const CPDF_PAPERSIZE_A1LANDSCAPE: CpdfPaperSize = 7;
pub const CPDF_PAPERSIZE_A2LANDSCAPE: CpdfPaperSize = 8;
pub const CPDF_PAPERSIZE_A3LANDSCAPE: CpdfPaperSize = 9;
pub const CPDF_PAPERSIZE_A4LANDSCAPE: CpdfPaperSize = 10;
pub const CPDF_PAPERSIZE_A5LANDSCAPE: CpdfPaperSize = 11;
pub const CPDF_PAPERSIZE_USLETTERPORTRAIT: CpdfPaperSize = 12;
pub const CPDF_PAPERSIZE_USLETTERLANDSCAPE: CpdfPaperSize = 13;
pub const CPDF_PAPERSIZE_USLEGALPORTRAIT: CpdfPaperSize = 14;
pub const CPDF_PAPERSIZE_USLEGALLANDSCAPE: CpdfPaperSize = 15;
pub const CPDF_ANCHOR_POSCENTRE: CpdfAnchor = 0;
pub const CPDF_ANCHOR_POSLEFT: CpdfAnchor = 1;
pub const CPDF_ANCHOR_POSRIGHT: CpdfAnchor = 2;
pub const CPDF_ANCHOR_TOP: CpdfAnchor = 3;
pub const CPDF_ANCHOR_TOPLEFT: CpdfAnchor = 4;
pub const CPDF_ANCHOR_TOPRIGHT: CpdfAnchor = 5;
pub const CPDF_ANCHOR_LEFT: CpdfAnchor = 6;
pub const CPDF_ANCHOR_BOTTOMLEFT: CpdfAnchor = 7;
pub const CPDF_ANCHOR_BOTTOM: CpdfAnchor = 8;
pub const CPDF_ANCHOR_BOTTOMRIGHT: CpdfAnchor = 9;
pub const CPDF_ANCHOR_RIGHT: CpdfAnchor = 10;
pub const CPDF_ANCHOR_DIAGONAL: CpdfAnchor = 11;
pub const CPDF_ANCHOR_REVERSEDIAGONAL: CpdfAnchor = 12;
pub const CPDF_FONT_TIMESROMAN: CpdfFont = 0;
pub const CPDF_FONT_TIMESBOLD: CpdfFont = 1;
pub const CPDF_FONT_TIMESITALIC: CpdfFont = 2;
pub const CPDF_FONT_TIMESBOLDITALIC: CpdfFont = 3;
pub const CPDF_FONT_HELVETICA: CpdfFont = 4;
pub const CPDF_FONT_HELVETICABOLD: CpdfFont = 5;
pub const CPDF_FONT_HELVETICAOBLIQUE: CpdfFont = 6;
pub const CPDF_FONT_HELVETICABOLDOBLIQUE: CpdfFont = 7;
pub const CPDF_FONT_COURIER: CpdfFont = 8;
pub const CPDF_FONT_COURIERBOLD: CpdfFont = 9;
pub const CPDF_FONT_COURIEROBLIQUE: CpdfFont = 10;
pub const CPDF_FONT_COURIERBOLDOBLIQUE: CpdfFont = 11;
pub const CPDF_JUSTIFICATION_LEFTJUSTIFY: CpdfJustification = 0;
pub const CPDF_JUSTIFICATION_CENTREJUSTIFY: CpdfJustification = 1;
pub const CPDF_JUSTIFICATION_RIGHTJUSTIFY: CpdfJustification = 2;
pub const CPDF_LAYOUT_SINGLEPAGE: CpdfLayout = 0;
pub const CPDF_LAYOUT_ONECOLUMN: CpdfLayout = 1;
pub const CPDF_LAYOUT_TWOCOLUMNLEFT: CpdfLayout = 2;
pub const CPDF_LAYOUT_TWOCOLUMNRIGHT: CpdfLayout = 3;
pub const CPDF_LAYOUT_TWOPAGELEFT: CpdfLayout = 4;
pub const CPDF_LAYOUT_TWOPAGERIGHT: CpdfLayout = 5;
pub const CPDF_PAGEMODE_USENONE: CpdfPageMode = 0;
pub const CPDF_PAGEMODE_USEOUTLINES: CpdfPageMode = 1;
pub const CPDF_PAGEMODE_USETHUMBS: CpdfPageMode = 2;
pub const CPDF_PAGEMODE_USEOC: CpdfPageMode = 3;
pub const CPDF_PAGEMODE_USEATTACHMENTS: CpdfPageMode = 4;
pub const CPDF_PAGELABELSTYLE_DECIMALARABIC: CpdfPageLabelStyle = 0;
pub const CPDF_PAGELABELSTYLE_UPPERCASEROMAN: CpdfPageLabelStyle = 1;
pub const CPDF_PAGELABELSTYLE_LOWERCASEROMAN: CpdfPageLabelStyle = 2;
pub const CPDF_PAGELABELSTYLE_UPPERCASELETTERS: CpdfPageLabelStyle = 3;
pub const CPDF_PAGELABELSTYLE_LOWERCASELETTERS: CpdfPageLabelStyle = 4;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CpdfPosition {
    pub cpdf_anchor: c_int,
    pub cpdf_coord1: f64,
    pub cpdf_coord2: f64,
}

extern "C" {
    pub fn cpdf_startup(arg1: *const *const c_char);
    pub fn cpdf_version() -> *mut c_char;
    pub fn cpdf_setFast();
    pub fn cpdf_setSlow();
    pub static mut cpdf_lastError: c_int;
    pub static mut cpdf_lastErrorString: *mut c_char;
    pub fn cpdf_fLastError() -> c_int;
    pub fn cpdf_fLastErrorString() -> *mut c_char;
    pub fn cpdf_clearError();
    pub fn cpdf_onExit();
    pub fn cpdf_fromFile(arg1: *const c_char, arg2: *const c_char) -> c_int;
    pub fn cpdf_fromFileLazy(arg1: *const c_char, arg2: *const c_char) -> c_int;
    pub fn cpdf_fromMemory(arg1: *mut c_void, arg2: c_int, arg3: *const c_char) -> c_int;
    pub fn cpdf_fromMemoryLazy(arg1: *mut c_void, arg2: c_int, arg3: *const c_char) -> c_int;
    pub fn cpdf_deletePdf(arg1: c_int);
    pub fn cpdf_replacePdf(arg1: c_int, arg2: c_int);
    pub fn cpdf_startEnumeratePDFs() -> c_int;
    pub fn cpdf_enumeratePDFsKey(arg1: c_int) -> c_int;
    pub fn cpdf_enumeratePDFsInfo(arg1: c_int) -> *mut c_char;
    pub fn cpdf_endEnumeratePDFs();
    pub fn cpdf_ptOfCm(arg1: f64) -> f64;
    pub fn cpdf_ptOfMm(arg1: f64) -> f64;
    pub fn cpdf_ptOfIn(arg1: f64) -> f64;
    pub fn cpdf_cmOfPt(arg1: f64) -> f64;
    pub fn cpdf_mmOfPt(arg1: f64) -> f64;
    pub fn cpdf_inOfPt(arg1: f64) -> f64;
    pub fn cpdf_parsePagespec(arg1: c_int, arg2: *const c_char) -> c_int;
    pub fn cpdf_validatePagespec(arg1: *const c_char) -> c_int;
    pub fn cpdf_stringOfPagespec(arg1: c_int, arg2: c_int) -> *mut c_char;
    pub fn cpdf_blankRange() -> c_int;
    pub fn cpdf_deleteRange(arg1: c_int);
    pub fn cpdf_range(arg1: c_int, arg2: c_int) -> c_int;
    pub fn cpdf_all(arg1: c_int) -> c_int;
    pub fn cpdf_even(arg1: c_int) -> c_int;
    pub fn cpdf_odd(arg1: c_int) -> c_int;
    pub fn cpdf_rangeUnion(arg1: c_int, arg2: c_int) -> c_int;
    pub fn cpdf_difference(arg1: c_int, arg2: c_int) -> c_int;
    pub fn cpdf_removeDuplicates(arg1: c_int) -> c_int;
    pub fn cpdf_rangeLength(arg1: c_int) -> c_int;
    pub fn cpdf_rangeGet(arg1: c_int, arg2: c_int) -> c_int;
    pub fn cpdf_rangeAdd(arg1: c_int, arg2: c_int) -> c_int;
    pub fn cpdf_isInRange(arg1: c_int, arg2: c_int) -> c_int;
    pub fn cpdf_pages(arg1: c_int) -> c_int;
    pub fn cpdf_pagesFast(arg1: *const c_char, arg2: *const c_char) -> c_int;
    pub fn cpdf_toFile(arg1: c_int, arg2: *const c_char, arg3: c_int, arg4: c_int);
    pub fn cpdf_toFileExt(
        arg1: c_int,
        arg2: *const c_char,
        arg3: c_int,
        arg4: c_int,
        arg5: c_int,
        arg6: c_int,
        arg7: c_int,
    );
    pub fn cpdf_toMemory(arg1: c_int, arg2: c_int, arg3: c_int, arg4: *mut c_int) -> *mut c_void;
    pub fn cpdf_free(arg: *mut c_void);
    pub fn cpdf_isEncrypted(arg1: c_int) -> c_int;
    pub fn cpdf_decryptPdf(arg1: c_int, arg2: *const c_char);
    pub fn cpdf_decryptPdfOwner(arg1: c_int, arg2: *const c_char);
    pub fn cpdf_toFileEncrypted(
        arg1: c_int,
        arg2: c_int,
        arg3: *mut c_int,
        arg4: c_int,
        arg5: *const c_char,
        arg6: *const c_char,
        arg7: c_int,
        arg8: c_int,
        arg9: *const c_char,
    );
    pub fn cpdf_toFileEncryptedExt(
        arg1: c_int,
        arg2: c_int,
        arg3: *mut c_int,
        arg4: c_int,
        arg5: *const c_char,
        arg6: *const c_char,
        arg7: c_int,
        arg8: c_int,
        arg9: c_int,
        arg10: c_int,
        arg11: c_int,
        arg12: *const c_char,
    );
    pub fn cpdf_hasPermission(arg1: c_int, arg2: CpdfPermission) -> c_int;
    pub fn cpdf_encryptionKind(arg1: c_int) -> CpdfEncryptionMethod;
    pub fn cpdf_mergeSimple(arg1: *mut c_int, arg2: c_int) -> c_int;
    pub fn cpdf_merge(arg1: *mut c_int, arg2: c_int, arg3: c_int, arg4: c_int) -> c_int;
    pub fn cpdf_mergeSame(
        arg1: *mut c_int,
        arg2: c_int,
        arg3: c_int,
        arg4: c_int,
        arg5: *mut c_int,
    ) -> c_int;
    pub fn cpdf_selectPages(arg1: c_int, arg2: c_int) -> c_int;
    pub fn cpdf_scalePages(arg1: c_int, arg2: c_int, arg3: f64, arg4: f64);
    pub fn cpdf_scaleToFit(arg1: c_int, arg2: c_int, arg3: f64, arg4: f64, arg5: f64);
    pub fn cpdf_scaleToFitPaper(arg1: c_int, arg2: c_int, arg3: CpdfPaperSize, arg4: f64);

    pub fn cpdf_scaleContents(arg1: c_int, arg2: c_int, arg3: CpdfPosition, arg4: f64);
    pub fn cpdf_shiftContents(arg1: c_int, arg2: c_int, arg3: f64, arg4: f64);
    pub fn cpdf_rotate(arg1: c_int, arg2: c_int, arg3: c_int);
    pub fn cpdf_rotateBy(arg1: c_int, arg2: c_int, arg3: c_int);
    pub fn cpdf_rotateContents(arg1: c_int, arg2: c_int, arg3: f64);
    pub fn cpdf_upright(arg1: c_int, arg2: c_int);
    pub fn cpdf_hFlip(arg1: c_int, arg2: c_int);
    pub fn cpdf_vFlip(arg1: c_int, arg2: c_int);
    pub fn cpdf_crop(arg1: c_int, arg2: c_int, arg3: f64, arg4: f64, arg5: f64, arg6: f64);
    pub fn cpdf_removeCrop(arg1: c_int, arg2: c_int);
    pub fn cpdf_removeTrim(arg1: c_int, arg2: c_int);
    pub fn cpdf_removeArt(arg1: c_int, arg2: c_int);
    pub fn cpdf_removeBleed(arg1: c_int, arg2: c_int);
    pub fn cpdf_trimMarks(arg1: c_int, arg2: c_int);
    pub fn cpdf_showBoxes(arg1: c_int, arg2: c_int);
    pub fn cpdf_hardBox(arg1: c_int, arg2: c_int, arg3: *const c_char);
    pub fn cpdf_compress(arg1: c_int);
    pub fn cpdf_decompress(arg1: c_int);
    pub fn cpdf_squeezeInMemory(arg1: c_int);
    pub fn cpdf_startGetBookmarkInfo(arg1: c_int);
    pub fn cpdf_numberBookmarks() -> c_int;
    pub fn cpdf_getBookmarkLevel(arg1: c_int) -> c_int;
    pub fn cpdf_getBookmarkPage(arg1: c_int, arg2: c_int) -> c_int;
    pub fn cpdf_getBookmarkText(arg1: c_int) -> *mut c_char;
    pub fn cpdf_getBookmarkOpenStatus(arg1: c_int) -> c_int;
    pub fn cpdf_endGetBookmarkInfo();
    pub fn cpdf_startSetBookmarkInfo(arg1: c_int);
    pub fn cpdf_setBookmarkLevel(arg1: c_int, arg2: c_int);
    pub fn cpdf_setBookmarkPage(arg1: c_int, arg2: c_int, arg3: c_int);
    pub fn cpdf_setBookmarkOpenStatus(arg1: c_int, arg2: c_int);
    pub fn cpdf_setBookmarkText(arg1: c_int, arg2: *const c_char);
    pub fn cpdf_endSetBookmarkInfo(arg1: c_int);
    pub fn cpdf_getBookmarksJSON(arg1: c_int, arg2: *mut c_int) -> *mut c_void;
    pub fn cpdf_setBookmarksJSON(arg1: c_int, arg2: *mut c_void, arg3: c_int);
    pub fn cpdf_tableOfContents(
        arg1: c_int,
        arg2: c_int,
        arg3: f64,
        arg4: *const c_char,
        arg5: c_int,
    );
    pub fn cpdf_stampOn(arg1: c_int, arg2: c_int, arg3: c_int);
    pub fn cpdf_stampUnder(arg1: c_int, arg2: c_int, arg3: c_int);
    pub fn cpdf_stampExtended(
        arg1: c_int,
        arg2: c_int,
        arg3: c_int,
        arg4: c_int,
        arg5: c_int,
        arg6: CpdfPosition,
        arg7: c_int,
    );
    pub fn cpdf_combinePages(arg1: c_int, arg2: c_int) -> c_int;
    pub fn cpdf_addText(
        arg1: c_int,
        arg2: c_int,
        arg3: c_int,
        arg4: *const c_char,
        arg5: CpdfPosition,
        arg6: f64,
        arg7: c_int,
        arg8: CpdfFont,
        arg9: f64,
        arg10: f64,
        arg11: f64,
        arg12: f64,
        arg13: c_int,
        arg14: c_int,
        arg15: c_int,
        arg16: f64,
        arg17: CpdfJustification,
        arg18: c_int,
        arg19: c_int,
        arg20: *const c_char,
        arg21: f64,
        arg22: c_int,
    );
    pub fn cpdf_addTextSimple(
        arg1: c_int,
        arg2: c_int,
        arg3: *const c_char,
        arg4: CpdfPosition,
        arg5: CpdfFont,
        arg6: f64,
    );
    pub fn cpdf_removeText(arg1: c_int, arg2: c_int);
    pub fn cpdf_textWidth(arg1: CpdfFont, arg2: *const c_char) -> c_int;
    pub fn cpdf_addContent(arg1: *const c_char, arg2: c_int, arg3: c_int, arg4: c_int);
    pub fn cpdf_stampAsXObject(arg1: c_int, arg2: c_int, arg3: c_int) -> *mut c_char;
    pub fn cpdf_impose(
        arg1: c_int,
        arg2: f64,
        arg3: f64,
        arg4: c_int,
        arg5: c_int,
        arg6: c_int,
        arg7: c_int,
        arg8: c_int,
        arg9: f64,
        arg10: f64,
        arg11: f64,
    );
    pub fn cpdf_twoUp(arg1: c_int);
    pub fn cpdf_twoUpStack(arg1: c_int);
    pub fn cpdf_padBefore(arg1: c_int, arg2: c_int);
    pub fn cpdf_padAfter(arg1: c_int, arg2: c_int);
    pub fn cpdf_padEvery(arg1: c_int, arg2: c_int);
    pub fn cpdf_padMultiple(arg1: c_int, arg2: c_int);
    pub fn cpdf_padMultipleBefore(arg1: c_int, arg2: c_int);
    pub fn cpdf_annotationsJSON(arg1: c_int, arg2: *mut c_int) -> *mut c_void;
    pub fn cpdf_isLinearized(arg1: *const c_char) -> c_int;
    pub fn cpdf_getVersion(arg1: c_int) -> c_int;
    pub fn cpdf_getMajorVersion(arg1: c_int) -> c_int;
    pub fn cpdf_getTitle(arg1: c_int) -> *mut c_char;
    pub fn cpdf_getAuthor(arg1: c_int) -> *mut c_char;
    pub fn cpdf_getSubject(arg1: c_int) -> *mut c_char;
    pub fn cpdf_getKeywords(arg1: c_int) -> *mut c_char;
    pub fn cpdf_getCreator(arg1: c_int) -> *mut c_char;
    pub fn cpdf_getProducer(arg1: c_int) -> *mut c_char;
    pub fn cpdf_getCreationDate(arg1: c_int) -> *mut c_char;
    pub fn cpdf_getModificationDate(arg1: c_int) -> *mut c_char;
    pub fn cpdf_getTitleXMP(arg1: c_int) -> *mut c_char;
    pub fn cpdf_getAuthorXMP(arg1: c_int) -> *mut c_char;
    pub fn cpdf_getSubjectXMP(arg1: c_int) -> *mut c_char;
    pub fn cpdf_getKeywordsXMP(arg1: c_int) -> *mut c_char;
    pub fn cpdf_getCreatorXMP(arg1: c_int) -> *mut c_char;
    pub fn cpdf_getProducerXMP(arg1: c_int) -> *mut c_char;
    pub fn cpdf_getCreationDateXMP(arg1: c_int) -> *mut c_char;
    pub fn cpdf_getModificationDateXMP(arg1: c_int) -> *mut c_char;
    pub fn cpdf_setTitle(arg1: c_int, arg2: *const c_char);
    pub fn cpdf_setAuthor(arg1: c_int, arg2: *const c_char);
    pub fn cpdf_setSubject(arg1: c_int, arg2: *const c_char);
    pub fn cpdf_setKeywords(arg1: c_int, arg2: *const c_char);
    pub fn cpdf_setCreator(arg1: c_int, arg2: *const c_char);
    pub fn cpdf_setProducer(arg1: c_int, arg2: *const c_char);
    pub fn cpdf_setCreationDate(arg1: c_int, arg2: *const c_char);
    pub fn cpdf_setModificationDate(arg1: c_int, arg2: *const c_char);
    pub fn cpdf_setTitleXMP(arg1: c_int, arg2: *const c_char);
    pub fn cpdf_setAuthorXMP(arg1: c_int, arg2: *const c_char);
    pub fn cpdf_setSubjectXMP(arg1: c_int, arg2: *const c_char);
    pub fn cpdf_setKeywordsXMP(arg1: c_int, arg2: *const c_char);
    pub fn cpdf_setCreatorXMP(arg1: c_int, arg2: *const c_char);
    pub fn cpdf_setProducerXMP(arg1: c_int, arg2: *const c_char);
    pub fn cpdf_setCreationDateXMP(arg1: c_int, arg2: *const c_char);
    pub fn cpdf_setModificationDateXMP(arg1: c_int, arg2: *const c_char);
    pub fn cpdf_getDateComponents(
        arg1: *const c_char,
        arg2: *mut c_int,
        arg3: *mut c_int,
        arg4: *mut c_int,
        arg5: *mut c_int,
        arg6: *mut c_int,
        arg7: *mut c_int,
        arg8: *mut c_int,
        arg9: *mut c_int,
    );
    pub fn cpdf_dateStringOfComponents(
        arg1: c_int,
        arg2: c_int,
        arg3: c_int,
        arg4: c_int,
        arg5: c_int,
        arg6: c_int,
        arg7: c_int,
        arg8: c_int,
    ) -> *mut c_char;
    pub fn cpdf_getPageRotation(arg1: c_int, arg2: c_int) -> c_int;
    pub fn cpdf_hasBox(arg1: c_int, arg2: c_int, arg3: *const c_char) -> c_int;
    pub fn cpdf_getMediaBox(
        arg1: c_int,
        arg2: c_int,
        arg3: *mut f64,
        arg4: *mut f64,
        arg5: *mut f64,
        arg6: *mut f64,
    );
    pub fn cpdf_getCropBox(
        arg1: c_int,
        arg2: c_int,
        arg3: *mut f64,
        arg4: *mut f64,
        arg5: *mut f64,
        arg6: *mut f64,
    );
    pub fn cpdf_getTrimBox(
        arg1: c_int,
        arg2: c_int,
        arg3: *mut f64,
        arg4: *mut f64,
        arg5: *mut f64,
        arg6: *mut f64,
    );
    pub fn cpdf_getArtBox(
        arg1: c_int,
        arg2: c_int,
        arg3: *mut f64,
        arg4: *mut f64,
        arg5: *mut f64,
        arg6: *mut f64,
    );
    pub fn cpdf_getBleedBox(
        arg1: c_int,
        arg2: c_int,
        arg3: *mut f64,
        arg4: *mut f64,
        arg5: *mut f64,
        arg6: *mut f64,
    );
    pub fn cpdf_setMediabox(arg1: c_int, arg2: c_int, arg3: f64, arg4: f64, arg5: f64, arg6: f64);
    pub fn cpdf_setCropBox(arg1: c_int, arg2: c_int, arg3: f64, arg4: f64, arg5: f64, arg6: f64);
    pub fn cpdf_setTrimBox(arg1: c_int, arg2: c_int, arg3: f64, arg4: f64, arg5: f64, arg6: f64);
    pub fn cpdf_setArtBox(arg1: c_int, arg2: c_int, arg3: f64, arg4: f64, arg5: f64, arg6: f64);
    pub fn cpdf_setBleedBox(arg1: c_int, arg2: c_int, arg3: f64, arg4: f64, arg5: f64, arg6: f64);
    pub fn cpdf_markTrapped(arg1: c_int);
    pub fn cpdf_markUntrapped(arg1: c_int);
    pub fn cpdf_markTrappedXMP(arg1: c_int);
    pub fn cpdf_markUntrappedXMP(arg1: c_int);

    pub fn cpdf_setPageLayout(arg1: c_int, arg2: CpdfLayout);

    pub fn cpdf_setPageMode(arg1: c_int, arg2: CpdfPageMode);
    pub fn cpdf_hideToolbar(arg1: c_int, arg2: c_int);
    pub fn cpdf_hideMenubar(arg1: c_int, arg2: c_int);
    pub fn cpdf_hideWindowUi(arg1: c_int, arg2: c_int);
    pub fn cpdf_fitWindow(arg1: c_int, arg2: c_int);
    pub fn cpdf_centerWindow(arg1: c_int, arg2: c_int);
    pub fn cpdf_displayDocTitle(arg1: c_int, arg2: c_int);
    pub fn cpdf_openAtPage(arg1: c_int, arg2: c_int, arg3: c_int);
    pub fn cpdf_setMetadataFromFile(arg1: c_int, arg2: *const c_char);
    pub fn cpdf_setMetadataFromByteArray(arg1: c_int, arg2: *mut c_void, arg3: c_int);
    pub fn cpdf_getMetadata(arg1: c_int, arg2: *mut c_int) -> *mut c_void;
    pub fn cpdf_removeMetadata(arg1: c_int);
    pub fn cpdf_createMetadata(arg1: c_int);
    pub fn cpdf_setMetadataDate(arg1: c_int, arg2: *const c_char);

    pub fn cpdf_addPageLabels(
        arg1: c_int,
        arg2: CpdfPageLabelStyle,
        arg3: *const c_char,
        arg4: c_int,
        arg5: c_int,
        arg6: c_int,
    );
    pub fn cpdf_removePageLabels(arg1: c_int);
    pub fn cpdf_getPageLabelStringForPage(arg1: c_int, arg2: c_int) -> *mut c_char;
    pub fn cpdf_startGetPageLabels(arg1: c_int) -> c_int;
    pub fn cpdf_getPageLabelStyle(arg1: c_int) -> CpdfPageLabelStyle;
    pub fn cpdf_getPageLabelPrefix(arg1: c_int) -> *mut c_char;
    pub fn cpdf_getPageLabelOffset(arg1: c_int) -> c_int;
    pub fn cpdf_getPageLabelRange(arg1: c_int) -> c_int;
    pub fn cpdf_endGetPageLabels();
    pub fn cpdf_attachFile(arg1: *const c_char, arg2: c_int);
    pub fn cpdf_attachFileToPage(arg1: *const c_char, arg2: c_int, arg3: c_int);
    pub fn cpdf_attachFileFromMemory(
        arg1: *mut c_void,
        arg2: c_int,
        arg3: *const c_char,
        arg4: c_int,
    );
    pub fn cpdf_attachFileToPageFromMemory(
        arg1: *mut c_void,
        arg2: c_int,
        arg3: *const c_char,
        arg4: c_int,
        arg5: c_int,
    );
    pub fn cpdf_removeAttachedFiles(arg1: c_int);
    pub fn cpdf_startGetAttachments(arg1: c_int);
    pub fn cpdf_numberGetAttachments() -> c_int;
    pub fn cpdf_getAttachmentName(arg1: c_int) -> *mut c_char;
    pub fn cpdf_getAttachmentPage(arg1: c_int) -> c_int;
    pub fn cpdf_getAttachmentData(arg1: c_int, arg2: *mut c_int) -> *mut c_void;
    pub fn cpdf_endGetAttachments();
    pub fn cpdf_startGetImageResolution(arg1: c_int, arg2: f32) -> c_int;
    pub fn cpdf_getImageResolutionPageNumber(arg1: c_int) -> c_int;
    pub fn cpdf_getImageResolutionImageName(arg1: c_int) -> *mut c_char;
    pub fn cpdf_getImageResolutionXPixels(arg1: c_int) -> c_int;
    pub fn cpdf_getImageResolutionYPixels(arg1: c_int) -> c_int;
    pub fn cpdf_getImageResolutionXRes(arg1: c_int) -> f64;
    pub fn cpdf_getImageResolutionYRes(arg1: c_int) -> f64;
    pub fn cpdf_endGetImageResolution();
    pub fn cpdf_startGetFontInfo(arg1: c_int);
    pub fn cpdf_numberFonts() -> c_int;
    pub fn cpdf_getFontPage(arg1: c_int) -> c_int;
    pub fn cpdf_getFontName(arg1: c_int) -> *mut c_char;
    pub fn cpdf_getFontType(arg1: c_int) -> *mut c_char;
    pub fn cpdf_getFontEncoding(arg1: c_int) -> *mut c_char;
    pub fn cpdf_endGetFontInfo();
    pub fn cpdf_removeFonts(arg1: c_int);
    pub fn cpdf_copyFont(arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: *const c_char);
    pub fn cpdf_outputJSON(arg1: *const c_char, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int);
    pub fn cpdf_outputJSONMemory(
        arg1: c_int,
        arg2: c_int,
        arg3: c_int,
        arg4: c_int,
        arg5: *mut c_int,
    ) -> *mut c_void;
    pub fn cpdf_fromJSON(arg1: *const c_char) -> c_int;
    pub fn cpdf_fromJSONMemory(arg1: *mut c_void, arg2: c_int) -> c_int;
    pub fn cpdf_startGetOCGList(pdf: c_int) -> c_int;
    pub fn cpdf_OCGListEntry(i: c_int) -> *mut c_char;
    pub fn cpdf_endGetOCGList();
    pub fn cpdf_OCGRename(arg1: c_int, arg2: *const c_char, arg3: *const c_char);
    pub fn cpdf_OCGOrderAll(arg1: c_int);
    pub fn cpdf_OCGCoalesce(arg1: c_int);
    pub fn cpdf_blankDocument(arg1: f64, arg2: f64, arg3: c_int) -> c_int;
    pub fn cpdf_blankDocumentPaper(arg1: CpdfPaperSize, arg2: c_int) -> c_int;
    pub fn cpdf_textToPDF(
        arg1: f64,
        arg2: f64,
        arg3: c_int,
        arg4: f64,
        arg5: *const c_char,
    ) -> c_int;
    pub fn cpdf_textToPDFPaper(arg1: c_int, arg2: c_int, arg3: f64, arg4: *const c_char) -> c_int;
    pub fn cpdf_draft(arg1: c_int, arg2: c_int, arg3: c_int);
    pub fn cpdf_removeAllText(arg1: c_int, arg2: c_int);
    pub fn cpdf_blackText(arg1: c_int, arg2: c_int);
    pub fn cpdf_blackLines(arg1: c_int, arg2: c_int);
    pub fn cpdf_blackFills(arg1: c_int, arg2: c_int);
    pub fn cpdf_thinLines(arg1: c_int, arg2: c_int, arg3: f64);
    pub fn cpdf_copyId(arg1: c_int, arg2: c_int);
    pub fn cpdf_removeId(arg1: c_int);
    pub fn cpdf_setVersion(arg1: c_int, arg2: c_int);
    pub fn cpdf_setFullVersion(arg1: c_int, arg2: c_int, arg3: c_int);
    pub fn cpdf_removeDictEntry(arg1: c_int, arg2: *const c_char);
    pub fn cpdf_removeDictEntrySearch(arg1: c_int, arg2: *const c_char, arg3: *const c_char);
    pub fn cpdf_replaceDictEntry(arg1: c_int, arg2: *const c_char, arg3: *const c_char);
    pub fn cpdf_replaceDictEntrySearch(
        arg1: c_int,
        arg2: *const c_char,
        arg3: *const c_char,
        arg4: *const c_char,
    );
    pub fn cpdf_getDictEntries(arg1: c_int, arg2: *const c_char, retlen: *mut c_int)
        -> *mut c_void;
    pub fn cpdf_removeClipping(arg1: c_int, arg2: c_int);
}
