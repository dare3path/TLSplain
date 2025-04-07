# TLSplain

This repo contains a proof-of-concept patch for openssl, a rust TLS client and a rust non-TLS server.  
You can run this non-TLS server then try to connect to it via the TLS client and see the failing errors.  
Without the patch you'd see errors like this:  
  
> Caused by:
>    0: client error (Connect)
>    1: error:0A00010B:SSL routines:tls_validate_record_header:wrong version number:ssl/record/methods/tlsany_meth.c:80:
>    2: error:0A00010B:SSL routines:tls_validate_record_header:wrong version number:ssl/record/methods/tlsany_meth.c:80:
  
and  
  
> Caused by:
>    0: client error (Connect)
>    1: error:0A0000C6:SSL routines:tls_get_more_records:packet length too long:ssl/record/methods/tls_common.c:662:, error:0A000139:SSL routines::record layer failure:ssl/record/rec_layer_s3.c:688:
>    2: error:0A0000C6:SSL routines:tls_get_more_records:packet length too long:ssl/record/methods/tls_common.c:662:, error:0A000139:SSL routines::record layer failure:ssl/record/rec_layer_s3.c:688:
  

but **with patch** more descriptive errors:  
  
> Caused by:
>    0: client error (Connect)
>    1: error:0A0007A0:SSL routines:tls_get_more_records:you attempted a TLS connection to a plaintext HTTP server ie. to a non-TLS server:ssl/record/methods/tls_common.c:747:, error:0A000139:SSL routines::record layer failure:ssl/record/rec_layer_s3.c:689:
>    2: error:0A0007A0:SSL routines:tls_get_more_records:you attempted a TLS connection to a plaintext HTTP server ie. to a non-TLS server:ssl/record/methods/tls_common.c:747:, error:0A000139:SSL routines::record layer failure:ssl/record/rec_layer_s3.c:689:
  
and  
  
> Caused by:
>    0: client error (Connect)
>    1: error:0A00079F:SSL routines:tls_get_more_records:you attempted a TLS connection to a non TLS server which is also non HTTP:ssl/record/methods/tls_common.c:752:, error:0A000139:SSL routines::record layer failure:ssl/record/rec_layer_s3.c:689:
>    2: error:0A00079F:SSL routines:tls_get_more_records:you attempted a TLS connection to a non TLS server which is also non HTTP:ssl/record/methods/tls_common.c:752:, error:0A000139:SSL routines::record layer failure:ssl/record/rec_layer_s3.c:689:

  
and  
  
> Caused by:
>    0: client error (Connect)
>    1: error:0A0007A1:SSL routines:tls_get_more_records:you attempted a TLS connection to a non-TLS server which replied with less than 5 bytes:ssl/record/methods/tls_common.c:659:, error:0A000139:SSL routines::record layer failure:ssl/record/rec_layer_s3.c:689:
>    2: error:0A0007A1:SSL routines:tls_get_more_records:you attempted a TLS connection to a non-TLS server which replied with less than 5 bytes:ssl/record/methods/tls_common.c:659:, error:0A000139:SSL routines::record layer failure:ssl/record/rec_layer_s3.c:689:
  
  
**Pros**
Seeing such errors might save you some time because you're not chasing ghost possibilities (like wondering what's wrong with your client).  

**Cons**
- if you've somehow trapped those specific errors before error:0A00010B:SSL and error:0A0000C6:SSL as a way to know if you're connecting to non-TLS server(s) then you'd have to change or add these new ones (depending on the patch), but seen above as: error:0A0007A0:SSL and error:0A00079F:SSL and error:0A0007A1:SSL

Initially I tried to report it here: https://github.com/hyperium/hyper/issues/3866  
But then I've realized it's in fact openssl at heart and many rust programs can be helped by modding openssl instead.  
xAI's Grok3 and Grok2 were used as help.  

## Licensing
The entire `TLSplain` project is licensed under the [Apache License 2.0](LICENSE), matching OpenSSL’s terms. This includes the patch, demo server, and client code.

**Optional MIT License for Demo Code**: The demo(ie. non-patch) files are also available under the [MIT License](LICENSE.mit) at your discretion—feel free to use them under MIT if that suits your needs!

