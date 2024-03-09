@ECHO OFF


SET VC_BUILD_TOOLS=C:\Program Files (x86)\Microsoft Visual Studio\2019\BuildTools

SET GIT_RAW_URL=https://github.com/coherentgraphics/cpdflib-binary/raw/master




CALL "%VC_BUILD_TOOLS%\VC\Auxiliary\Build\vcvars64.bat"


MKDIR libs
lib /def:libcpdf.def /out:libs/libcpdf-x64.lib /machine:x64
lib /def:libcpdf.def /out:libs/libcpdf-x86.lib /machine:x86

 CURL -L %GIT_RAW_URL%/windows64/libcpdf.dll -o libs/libcpdf-x64.dll
 CURL -L %GIT_RAW_URL%/windows32/libcpdf.dll -o libs/libcpdf-x86.dll