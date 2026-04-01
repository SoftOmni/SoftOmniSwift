@echo off

where /q python
IF %ERRORLEVEL% EQU 0 (
    echo "python" command is available.
) ELSE (
    echo "python" command is NOT available.
    exit /b 1
)
where /q clang
IF %ERRORLEVEL% EQU 0 (
    echo "clang" command is available.
) ELSE (
    echo "clang" command is NOT available.
    exit /b 1
)
python setup.py

