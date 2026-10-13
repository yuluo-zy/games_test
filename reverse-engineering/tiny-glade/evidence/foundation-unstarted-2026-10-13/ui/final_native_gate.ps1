& 'D:/game/reverse-engineering/tiny-glade/evidence/foundation-unstarted-2026-10-13/ui/build_native.ps1' -Action check -Packages 'tg-iced,tg-integrity-report,tg-gpu-selector,crash-reporter-gui'
if($LASTEXITCODE -ne 0){exit $LASTEXITCODE}
& 'D:/game/reverse-engineering/tiny-glade/evidence/foundation-unstarted-2026-10-13/ui/build_native.ps1' -Action clippy -Packages 'tg-iced,tg-integrity-report,tg-gpu-selector,crash-reporter-gui'
if($LASTEXITCODE -ne 0){exit $LASTEXITCODE}
& 'D:/game/reverse-engineering/tiny-glade/evidence/foundation-unstarted-2026-10-13/ui/build_native.ps1' -Action build -Packages 'tg-iced,tg-integrity-report,crash-reporter-gui'
exit $LASTEXITCODE
