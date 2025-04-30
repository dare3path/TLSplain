#!/usr/bin/bash

#openssl_ver="3.4.1"
openssl_ver="3.5.0"

#The arch linux repo dir:
arch_dir=~/"SOURCE/openssl/"

#unsure if both libs are needed:
lib1="${arch_dir}/src/openssl-${openssl_ver}/libcrypto.so.3"
lib2="${arch_dir}/src/openssl-${openssl_ver}/libssl.so.3"

