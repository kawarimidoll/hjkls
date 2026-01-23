" Test file for vim9script position errors
" vim9script must be at the start of the file

echo "some code before vim9script"
vim9script
echo "this won't work properly"
