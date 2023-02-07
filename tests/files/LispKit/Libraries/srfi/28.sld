;;; SRFI 28
;;; Basic format strings
;;;
;;; Many Scheme systems provide access to a function called format. This function takes as
;;; arguments a format string, an ordinary Scheme string containing zero or more escape
;;; sequences, followed zero or more Scheme values. The procedure processes the format
;;; string and performs string replacement on the escape sequences according to the rules
;;; for each code. This SRFI defines a basic version of format which should allow portable
;;; code to be written using the function without much (if any) effort on the part of
;;; Scheme implementors.
;;;
;;; Copyright © 2002 Scott G. Miller. All Rights Reserved.
;;;
;;; Permission is hereby granted, free of charge, to any person obtaining a copy of this
;;; software and associated documentation files (the "Software"), to deal in the Software
;;; without restriction, including without limitation the rights to use, copy, modify, merge,
;;; publish, distribute, sublicense, and/or sell copies of the Software, and to permit
;;; persons to whom the Software is furnished to do so, subject to the following conditions:
;;;
;;; The above copyright notice and this permission notice shall be included in all copies or
;;; substantial portions of the Software.
;;;
;;; THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
;;; INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR
;;; PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE
;;; FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
;;; OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
;;; DEALINGS IN THE SOFTWARE.
;;;
;;; Adaptation to LispKit
;;;   Copyright © 2017 Matthias Zenger. All rights reserved.

(define-library (srfi 28)

  (export format)
  
  (import (lispkit base))

  (begin
    (define (format formatstr . objects)
      (let ((buffer (open-output-string)))
        (let loop ((formatlst (string->list formatstr))
                   (objects objects))
          (cond ((null? formatlst)
                  (get-output-string buffer))
                ((char=? (car formatlst) #\~)
                  (if (null? (cdr formatlst))
                      (error "format: incomplete escape sequence")
                      (case (cadr formatlst)
                        ((#\a)
                          (if (null? objects)
                              (error "format: no value for escape sequence")
                              (begin (display (car objects) buffer)
                                     (loop (cddr formatlst) (cdr objects)))))
                        ((#\s)
                          (if (null? objects)
                              (error "format: no value for escape sequence")
                              (begin (write (car objects) buffer)
                                     (loop (cddr formatlst) (cdr objects)))))
                        ((#\%)
                          (newline buffer)
                          (loop (cddr formatlst) objects))
                        ((#\~)
                          (write-char #\~ buffer)
                          (loop (cddr formatlst) objects))
                        (else
                          (error "format: unrecognized escape sequence" (cadr formatlst))))))
                (else
                  (write-char (car formatlst) buffer)
                  (loop (cdr formatlst) objects))))))))
