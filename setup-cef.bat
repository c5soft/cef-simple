SET CEF_PATH=D:\Programs\CEF

SET dst=target\debug
mkdir %dst%
xcopy /Y %CEF_PATH%\Resources\* %dst%
xcopy /Y %CEF_PATH%\Release\* %dst%

SET dst=target\release
mkdir %dst%
xcopy /Y %CEF_PATH%\Resources\* %dst%
xcopy /Y %CEF_PATH%\Release\* %dst%


SET CEF_PATH=D:\Programs\CEF
SET LIB=D:\Programs\CEF\release