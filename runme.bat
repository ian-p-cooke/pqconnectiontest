@echo off
.\target\release\pqconectiontest.exe
if %errorlevel% NEQ 0 GOTO :Error
 
echo success.
GOTO :End
 
:Error
echo failure: %errorlevel%
 
:End