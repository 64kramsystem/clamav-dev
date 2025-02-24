# Copyright (C) 2020-2022 Cisco Systems, Inc. and/or its affiliates. All rights reserved.

#
# Example executables
#
add_executable(ex1)
target_sources(ex1
    PRIVATE   ex1.c)
set_target_properties( ex1 PROPERTIES COMPILE_FLAGS "${WARNCFLAGS}" )
target_link_libraries(ex1
    PRIVATE
        ClamAV::libclamav)
if(LLVM_FOUND)
    target_link_directories( ex1 PUBLIC ${LLVM_LIBRARY_DIRS} )
    target_link_libraries( ex1 PUBLIC ${LLVM_LIBRARIES} )
endif()

if(ENABLE_STATIC_LIB)
    add_executable(ex1_static)
    set_target_properties( ex1_static PROPERTIES COMPILE_FLAGS "${WARNCFLAGS}" )
    target_sources(ex1_static
        PRIVATE   ex1.c)
    target_link_libraries(ex1_static
        PRIVATE
            ClamAV::libclamav_static)
    if(LLVM_FOUND)
        target_link_directories( ex1_static PUBLIC ${LLVM_LIBRARY_DIRS} )
        target_link_libraries( ex1_static PUBLIC ${LLVM_LIBRARIES} )
    endif()
endif()
