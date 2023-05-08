@rem Copyright (C) Microsoft Corporation. All rights reserved.
@rem https://github.com/MicrosoftDocs/visualstudio-docs/blob/main/docs/install/advanced-build-tools-container.md#install-script
@rem Licensed under the MIT license.

@if not defined _echo echo off
setlocal enabledelayedexpansion

call %*
if "%ERRORLEVEL%"=="3010" (
    exit /b 0
) else (
    if not "%ERRORLEVEL%"=="0" (
        set ERR=%ERRORLEVEL%
        call C:\TEMP\collect.exe -zip:C:\vslogs.zip

        exit /b !ERR!
    )
)
