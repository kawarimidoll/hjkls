" Test file for hjkls:ignore directive

" Line 4: should trigger normal_bang warning (not ignored)
normal j

" hjkls:ignore-next-line suspicious#normal_bang
normal k

" Line 9: should trigger normal_bang warning (not ignored)
normal l

" hjkls:ignore suspicious#normal_bang
normal m
normal n

" Line 16: should still trigger double_dot hint (different rule)
echo "hello" . "world"

" hjkls:ignore style#double_dot
echo "foo" . "bar"
