;;; SRFI 144 REGRESSION TEST SUITE
;;;
;;; This is the test suite for SRFI 144.
;;;
;;; Implementation of original test suite:
;;; Copyright © 2017 William D Clinger. All rights reserved.
;;;
;;; Permission is hereby granted, free of charge, to any person obtaining a copy of this
;;; software and associated documentation files (the "Software"), to deal in the Software
;;; without restriction, including without limitation the rights to use, copy, modify,
;;; merge, publish, distribute, sublicense, and/or sell copies of the Software, and to
;;; permit persons to whom the Software is furnished to do so, subject to the following
;;; conditions:
;;;
;;; The above copyright notice and this permission notice shall be included in all copies
;;; or substantial portions of the Software.
;;;
;;; THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
;;; INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR
;;; PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
;;; LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT
;;; OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
;;; OTHER DEALINGS IN THE SOFTWARE.
;;;
;;; LispKit Port:
;;;   Copyright © 2022 Matthias Zenger. All rights reserved.

(import (lispkit base)
        (lispkit test)
        (srfi 144))

(test-begin "SRFI 144: Flonums")

;; convenient values for test cases

(define posints (map flonum '(1 2 3 4 5 10 65536 1e23)))
(define nats (cons (flonum 0) posints))
(define ints (append (map flonum '(-20 -8 -2 -1)) nats))
(define posfracs (map flonum '(1/1000 1/10 1/3 1/2)))
(define extremes (list (fl- fl-greatest) (fl- fl-least) fl-least fl-greatest))
(define infinities (map flonum (list -inf.0 +inf.0)))
(define weird (append infinities (list (flonum +nan.0))))

(define somereals (append (map flonum (list (fl- fl-greatest) -10 (fl- fl-least) 0))
                         posfracs
                         posints))
(define somereals+weird (append somereals weird))

(define negzero (flonum -0.0))
(define zero (flonum 0))
(define one (flonum 1))
(define two (flonum 2))
(define neginf (flonum -inf.0))
(define posinf (flonum +inf.0))
(define nan (flonum +nan.0))

(test-group "flonum/constants"
  (test 2.718281828459045235360287                    fl-e)
  (test 0.3678794411714423215955238                   fl-1/e)
  (test 7.389056098930650227230427                    fl-e-2)
  (test 2.1932800507380154566                         fl-e-pi/4)
  (test 1.4426950408889634073599246810018921374266    fl-log2-e)
  (test 0.4342944819032518276511289                   fl-log10-e)
  (test 0.6931471805599453094172321                   fl-log-2)
  (test 1.4426950408889634073599246810018921374266    fl-1/log-2)
  (test 1.0986122886681096913952452                   fl-log-3)
  (test 1.144729885849400174143427                    fl-log-pi)
  (test 2.3025850929940456840179915                   fl-log-10)
  (test 0.4342944819032518276511289189166050822944    fl-1/log-10)
  (test 3.1415926535897932384626433832795028841972    fl-pi)
  (test 0.3183098861837906715377675267450287240689    fl-1/pi)
  (test 6.283185307179586476925287                    fl-2pi)
  (test 1.570796326794896619231322                    fl-pi/2)
  (test 0.7853981633974483096156608                   fl-pi/4)
  (test 0.5641895835477562869480795                   (/ fl-2/sqrt-pi 2))
  (test 9.869604401089358618834491                    fl-pi-squared)
  (test 0.0174532925199432957692369076848861271344    fl-degree)
  (test .3183098861837906715377675                    (/ fl-2/pi 2))
  (test 1.4142135623730950488016887242096980785697    fl-sqrt-2)
  (test 1.7320508075688772935274463415058723669428    fl-sqrt-3)
  (test 2.2360679774997896964091736687311762354406    fl-sqrt-5)
  (test 3.1622776601683793319988935444327185337196    fl-sqrt-10)
  (test 1.4142135623730950488016887242096980785697    (* 2 fl-1/sqrt-2))
  (test 1.2599210498948731647672106072782283505703    fl-cbrt-2)
  (test 1.4422495703074083823216383107801095883919    fl-cbrt-3)
  (test 1.1892071150027210667174999705604759152930    fl-4thrt-2)
  (test 1.6180339887498948482045868343656381177203    fl-phi)
  (test 0.4812118250596034474977589134243684231352    fl-log-phi)
  (test 2.0780869212350275376013226061177957677422    fl-1/log-phi)
  (test 0.5772156649015328606065120900824024310422    fl-euler)
  (test 1.7810724179901979852365041031071795491696    fl-e-euler)
  (test 0.8414709848078965066525023216302989996226    fl-sin-1)
  (test 0.5403023058681397174009366074420766037323    fl-cos-1)
  (test 1.7724538509055160272981674833411451827975    fl-gamma-1/2)
  (test 2.6789385347077476336556929409746776441287    fl-gamma-1/3)
  (test 1.3541179394264004169452880281545137855193    fl-gamma-2/3)
  ;; Implementation Constants
  (test-assert (inexact? fl-greatest))
  (test-assert (inexact? fl-least))
  (test-assert (inexact? fl-epsilon))
  (test-assert (real? fl-greatest))
  (test-assert (real? fl-least))
  (test-assert (real? fl-epsilon))
  (test-assert (flonum? fl-greatest))
  (test-assert (flonum? fl-least))
  (test-assert (flonum? fl-epsilon))
  (test-assert (< 0.0 fl-least fl-epsilon 1.0 (+ 1.0 fl-epsilon) fl-greatest posinf))
  (test-assert (= (* 2 fl-greatest) posinf))
  (test-assert (= 1 (/ (+ 1 (+ 1.0 fl-epsilon)) 2)))
  (test-assert (= 0.0 (/ fl-least 2)))
  (test-assert (boolean? fl-fast-fl+*))
  (test-assert (exact-integer? fl-integer-exponent-zero))
  (test-assert (exact-integer? fl-integer-exponent-nan))
)

(test-group "flonum/constructors"
  (test-equal (flonum 3) (flonum 3.0))
  (test-equal somereals (map flonum somereals))
  (test-equal weird (map flonum weird))
  (test-equal somereals (map fladjacent somereals somereals))
  (test-equal weird (map fladjacent weird weird))
  (test-equal fl-least (fladjacent zero posinf))
  (test-equal (fl- fl-least) (fladjacent zero neginf))
  (test-equal (fl+ fl-least fl-least) (fladjacent fl-least posinf))
  (test-equal zero (fladjacent fl-least neginf))
  (test-equal negzero (fladjacent (fl- fl-least) posinf))
  (test-equal (fl* -2.0 fl-least) (fladjacent (fl- fl-least) neginf))
  (test-equal fl-least (fladjacent zero one))
  (test-equal (fl- fl-least) (fladjacent zero (fl- one)))
  (test-equal (fl+ fl-least fl-least) (fladjacent fl-least one))
  (test-equal zero (fladjacent fl-least (fl- one)))
  (test-equal negzero (fladjacent (fl- fl-least) one))
  (test-equal (fl* -2.0 fl-least) (fladjacent (fl- fl-least) (fl- one)))
  (test-equal fl-epsilon (fl- (fladjacent one fl-greatest) one))
  (test-equal (fl/ fl-epsilon 2.0) (fl- one (fladjacent one zero)))
  (test-equal fl-greatest (fladjacent posinf zero))
  (test-equal (fl- fl-greatest) (fladjacent neginf zero))
  (test-equal zero (flcopysign zero posinf))
  (test-equal negzero (flcopysign zero neginf))
  (test-equal zero (flcopysign zero one))
  (test-equal negzero (flcopysign zero (fl- one)))
  (test-equal one (flcopysign one fl-least))
  (test-equal (fl- one) (flcopysign one (fl- fl-greatest)))
  (test-equal one (flcopysign (fl- one) zero))
  (test-equal somereals (map flcopysign somereals somereals))
  (test-equal (map fl- somereals) (map flcopysign somereals (map fl- somereals)))
  (test-equal infinities (map flcopysign infinities infinities))
  (test-equal (reverse infinities) (map flcopysign infinities (reverse infinities)))
  (test-equal zero (make-flonum zero 12))
  (test-equal zero (make-flonum zero -24))
  (test-equal zero (make-flonum zero 0))
  (test-equal somereals (map make-flonum somereals (map (lambda (x) 0) somereals)))
  (test-equal (map (lambda (x) (fl* (flonum 4) x)) somereals)
              (map make-flonum somereals (map (lambda (x) 2) somereals)))
  (test-equal (map (lambda (x) (fl/ x (flonum 16))) somereals)
              (map make-flonum somereals (map (lambda (x) -4) somereals)))
  (test-equal posinf (make-flonum fl-greatest 1))
  (test-equal neginf (make-flonum (fl- fl-greatest) 1))
  (test-equal (fl/ fl-greatest two) (make-flonum fl-greatest -1))
  (test-equal (fl- (fl/ fl-greatest two)) (make-flonum (fl- fl-greatest) -1))
  (test-equal (fl* two fl-least) (make-flonum fl-least 1))
  (test-equal (fl- (fl* two fl-least)) (make-flonum (fl- fl-least) 1))
  (test-equal zero (make-flonum fl-least -1))
  (test-equal negzero (make-flonum (fl- fl-least) -1))
)

(test-group "flonum/accessors"
  (call-with-values
    (lambda () (flinteger-fraction 3.75))
    (lambda (q r)
      (test q (flonum 3))
      (test r (flonum .75))))
  (call-with-values
    (lambda () (flinteger-fraction -3.75))
    (lambda (q r)
      (test q (flonum -3))
      (test r (flonum -.75))))
  (test-equal (flonum 12.0) (flexponent (flexpt two (flonum 12))))
  (test-approx (flonum 12) (flexponent (flexpt two (flonum 12.5))))
  (test-equal (flonum -5.0) (flexponent (flexpt two (flonum -5))))
  (test-approx (flonum +4) (flexponent (flexpt two (flonum +4.5))))
  (test-approx (flonum -5) (flexponent (flexpt two (flonum -4.5))))
  (test 12 (flinteger-exponent (flexpt two (flonum 12))))
  (test 12 (flinteger-exponent (flexpt two (flonum 12.5))))
  (test -5 (flinteger-exponent (flexpt two (flonum -5))))
  (test -5 (flinteger-exponent (flexpt two (flonum -4.5))))
  (let* ((correct?
           (lambda (x y n)
             (or (fl=? x (* y (expt two n)))
                 (fl=? x (* 4.00 y (expt two (- n 2))))
                 (fl=? x (* 0.25 y (expt two (+ n 2)))))))
          (test-flnormalized-fraction-exponent
           (lambda (x)
             (call-with-values
              (lambda () (flnormalized-fraction-exponent x))
              (lambda (y n)
                (list (flonum? y)
                      (exact-integer? n)
                      (fl<=? (flonum 0.5) (flabs y))
                      (fl<? (flabs y) one)
                      (correct? x y n)))))))
    (test '(#t #t #f #t #t)
          (test-flnormalized-fraction-exponent zero))
    (test '(#t #t #f #t #t)
          (test-flnormalized-fraction-exponent negzero))
    (test '(#t #t #t #t #t)
          (test-flnormalized-fraction-exponent one))
    (test '(#t #t #t #t #t)
          (test-flnormalized-fraction-exponent two))
    (test '(#t #t #t #t #t)
          (test-flnormalized-fraction-exponent fl-least))
    (test '(#t #t #t #t #t)
          (test-flnormalized-fraction-exponent fl-greatest))
    (test '(#t #t #t #t #t)
          (test-flnormalized-fraction-exponent (fl- fl-least)))
    (test '(#t #t #t #t #t)
          (test-flnormalized-fraction-exponent (fl- fl-greatest)))
    (test '(#t #t #t #t #t)
          (test-flnormalized-fraction-exponent posinf))
    (test '(#t #t #t #t #t)
          (test-flnormalized-fraction-exponent neginf))
    (test '(#t #t #f #f #f)
          (test-flnormalized-fraction-exponent nan)))
  (test 0 (flsign-bit one))
  (test 0 (flsign-bit zero))
  (test 1 (flsign-bit negzero))
  (test 1 (flsign-bit (flonum -2)))
  (test 0 (flsign-bit posinf))
  (test 1 (flsign-bit neginf))
)

(test-group "flonum/predicates"
  (let ((alltrue  (map (lambda (x) #t) somereals))
        (allfalse (map (lambda (x) #f) somereals)))
    (test alltrue (map flonum? somereals))
    (test '(#t #t #t) (map flonum? weird))
    (test-not   (fl=? zero fl-least))
    (test-assert (fl=? fl-least fl-least))
    (test-not   (fl=? one fl-least))
    (test-assert (fl=? neginf neginf))
    (test-not   (fl=? neginf posinf))
    (test-not   (fl=? posinf neginf))
    (test-assert (fl=? posinf posinf))
    (test-not   (fl=? zero nan))
    (test-not   (fl=? nan one))
    (test alltrue (map fl=? somereals somereals))
    (test (cdr allfalse) (map fl=? somereals (cdr somereals)))
    (test (cdr allfalse) (map fl=? (cdr somereals) somereals))
    (test-assert (fl<? zero fl-least))
    (test-not   (fl<? fl-least fl-least))
    (test-not   (fl<? one fl-least))
    (test-not   (fl<? neginf neginf))
    (test-assert (fl<? neginf posinf))
    (test-not   (fl<? posinf neginf))
    (test-not   (fl<? posinf posinf))
    (test-not   (fl<? zero nan))
    (test-not   (fl<? nan one))
    (test allfalse (map fl<? somereals somereals))
    (test-equal (cdr alltrue) (map fl<? somereals (cdr somereals)))
    (test-equal (cdr allfalse) (map fl<? (cdr somereals) somereals))
    (test-not   (fl>? zero fl-least))
    (test-not   (fl>? fl-least fl-least))
    (test-assert (fl>? one fl-least))
    (test-not   (fl>? neginf neginf))
    (test-not   (fl>? neginf posinf))
    (test-assert (fl>? posinf neginf))
    (test-not   (fl>? posinf posinf))
    (test-not   (fl>? zero nan))
    (test-not   (fl>? nan one))
    (test allfalse (map fl>? somereals somereals))
    (test-equal (cdr allfalse) (map fl>? somereals (cdr somereals)))
    (test-equal (cdr alltrue) (map fl>? (cdr somereals) somereals))
    (test-assert (fl<=? zero fl-least))
    (test-assert (fl<=? fl-least fl-least))
    (test-not   (fl<=? one fl-least))
    (test-assert (fl<=? neginf neginf))
    (test-assert (fl<=? neginf posinf))
    (test-not   (fl<=? posinf neginf))
    (test-assert (fl<=? posinf posinf))
    (test-not   (fl<=? zero nan))
    (test-not   (fl<=? nan one))
    (test alltrue (map fl<=? somereals somereals))
    (test-equal (cdr alltrue) (map fl<=? somereals (cdr somereals)))
    (test-equal (cdr allfalse) (map fl<=? (cdr somereals) somereals))
    (test-not   (fl>=? zero fl-least))
    (test-assert (fl>=? fl-least fl-least))
    (test-assert (fl>=? one fl-least))
    (test-assert (fl>=? neginf neginf))
    (test-not   (fl>=? neginf posinf))
    (test-assert (fl>=? posinf neginf))
    (test-assert (fl>=? posinf posinf))
    (test-not   (fl>=? zero nan))
    (test-not   (fl>=? nan one))
    (test alltrue (map fl>=? somereals somereals))
    (test-equal (cdr allfalse) (map fl>=? somereals (cdr somereals)))
    (test-equal (cdr alltrue) (map fl>=? (cdr somereals) somereals))
    (test-not   (flunordered? zero fl-least))
    (test-not   (flunordered? fl-least fl-least))
    (test-not   (flunordered? one fl-least))
    (test-not   (flunordered? neginf neginf))
    (test-not   (flunordered? neginf posinf))
    (test-not   (flunordered? posinf neginf))
    (test-not   (flunordered? posinf posinf))
    (test-assert (flunordered? zero nan))
    (test-assert (flunordered? nan one))
    (test allfalse (map flunordered? somereals somereals))
    (test-equal (cdr allfalse) (map flunordered? somereals (cdr somereals)))
    (test-equal (cdr allfalse) (map flunordered? (cdr somereals) somereals)))
  (test neginf (flmax))
  (test zero (flmax zero))
  (test one (flmax zero one))
  (test one (flmax one zero))
  (test-equal (car (reverse somereals)) (apply flmax somereals))
  (test posinf (flmin))
  (test one (flmin one))
  (test zero (flmin zero one))
  (test zero (flmin one zero))
  (test-equal (car somereals) (apply flmin somereals))
  (test-equal (map flinteger? somereals)
              (map fl=?
                   somereals
                   (map flround somereals)))
  (test-not   (flzero? neginf))
  (test-not   (flzero? (fl- fl-least)))
  (test-assert (flzero? negzero))
  (test-assert (flzero? zero))
  (test-not   (flzero? fl-least))
  (test-not   (flzero? posinf))
  (test-not   (flpositive? neginf))
  (test-not   (flpositive? (fl- fl-least)))
  (test-not   (flpositive? negzero))
  (test-not   (flpositive? zero))
  (test-assert (flpositive? fl-least))
  (test-assert (flpositive? posinf))
  (test-assert (flnegative? neginf))
  (test-assert (flnegative? (fl- fl-least)))
  (test-not   (flnegative? negzero))    ; explicit in SRFI 144
  (test-not   (flnegative? zero))
  (test-not   (flnegative? fl-least))
  (test-not   (flnegative? posinf))
  (test-not   (flodd? zero))
  (test-assert (flodd? one))
  (test-assert (fleven? zero))
  (test-not   (fleven? one))
  (test-equal (map flfinite? somereals) (map (lambda (x) #t) somereals))
  (test-equal (map flfinite? weird) (map (lambda (x) #f) weird))
  (test-assert (flinfinite? neginf))
  (test-assert (flinfinite? posinf))
  (test-not   (flinfinite? nan))
  (test-equal (map (lambda (x) #f) somereals) (map flinfinite? somereals))
  (test-not   (flnan? neginf))
  (test-not   (flnan? posinf))
  (test-assert (flnan? nan))
  (test-equal (map (lambda (x) #f) somereals) (map flnan? somereals))
  (test-assert (flnormalized? fl-greatest))
  (test-not   (flnormalized? fl-least))
  (test-not   (fldenormalized? fl-greatest))
  (test-assert (fldenormalized? fl-least))
)

(test-group "flonum/arithmetic"
  (test zero (fl+))
  (test zero (fl+ zero))
  (test #t (flzero? (fl+ negzero)))
  (test one (fl+ one))
  (test two (fl+ one one))
  (test nan (fl+ nan one))
  (test nan (fl+ one nan))
  (test-equal (map (lambda (x) (fl* (flonum 3) x)) somereals)
              (map fl+ somereals somereals somereals))
  (test infinities (map fl+ infinities infinities))
  (test-equal (map (lambda (x) #t) infinities)
              (map flnan? (map fl+ infinities (reverse infinities))))
  (test one (fl*))
  (test zero (fl* zero))
  (test #t (flzero? (fl* negzero)))
  (test one (fl* one))
  (test one (fl* one one))
  (test nan (fl* nan one))
  (test nan (fl* one nan))
  ; (test-equal (map (lambda (x) (flonum (expt x 3))) somereals)
  ;             (map fl* somereals somereals somereals))
  (test-equal (map (lambda (x) posinf) infinities)
              (map fl* infinities infinities))
  (test-equal (map (lambda (x) neginf) infinities)
              (map fl* infinities (reverse infinities)))
  (let ((three (flonum 3))
        (four  (flonum 4))
        (five  (flonum 5))
        (x23   (flonum 23))
        (ten11 (flonum (expt 10 11)))
        (ten12 (flonum (expt 10 12))))
    (test x23 (fl+* four five three))
    (test-equal (flonum (+ (* (exact ten11) (exact ten12)) (exact one)))
                (fl+* ten11 ten12 one))
    (test-equal (flonum (+ (* (exact ten11) (exact ten12)) (exact (fl- one))))
                (fl+* ten11 ten12 (fl- one)))
    ;; FIXME: the following test assumes IEEE double precision,
    ;; in which (expt 10 23) lies exactly halfway between the
    ;; two nearest flonums.
    (test-not (fl=? (fl+* ten11 ten12 one) (fl+* ten11 ten12 (fl- one)))))
  (test-assert (flnan? (fl+* zero posinf one)))
  (test-assert (flnan? (fl+* zero neginf one)))
  (test-assert (flnan? (fl+* posinf zero one)))
  (test-assert (flnan? (fl+* neginf zero one)))
  (test-assert (flnan? (fl+* zero posinf nan)))
  (test-assert (flnan? (fl+* zero neginf nan)))
  (test-assert (flnan? (fl+* posinf zero nan)))
  (test-assert (flnan? (fl+* neginf zero nan)))
  (test neginf (fl+* fl-greatest fl-greatest neginf))
  (test posinf (fl+* fl-greatest (fl- fl-greatest) posinf))
  (test-assert (flnan? (fl+* nan one one)))
  (test-assert (flnan? (fl+* one nan one)))
  (test-assert (flnan? (fl+* one one nan)))
  (test negzero (fl- zero))
  (test zero (fl- negzero))
  (test-equal (flonum -1) (fl- one))
  (test zero (fl- one one))
  (test nan (fl- nan one))
  (test nan (fl- one nan))
  (test-equal (map (lambda (x) (if (eqv? x zero) zero (fl- x))) somereals)
              (map fl- somereals somereals somereals))
  (test '(#t #t) (map flnan? (map fl- infinities infinities)))
  (test infinities (map fl- infinities (reverse infinities)))
  (test posinf (fl/ zero))
  (test neginf (fl/ negzero))
  (test one (fl/ one))
  (test one (fl/ one one))
  (test nan (fl/ nan one))
  (test nan (fl/ one nan))
  (test-equal (map (lambda (x) (if (flzero? x) (fl/ zero zero) (fl/ x))) somereals)
              (map fl/ somereals somereals somereals))
  (test '(#t #t) (map flnan? (map fl/ infinities infinities)))
  (test '(#t #t) (map flnan? (map fl/ infinities (reverse infinities))))
  (test zero (flabs zero))
  (test zero (flabs negzero))
  (test one (flabs one))
  (test-equal (flonum 5.25) (flabs (flonum -5.25)))
  (test one (flabsdiff zero one))
  (test one (flabsdiff one zero))
  (test zero (flabsdiff one one))
  (test posinf (flabsdiff posinf neginf))
  (test posinf (flabsdiff neginf posinf))
  (test one (flsgn posinf))
  (test-equal (fl- one) (flsgn neginf))
  (test one (flsgn zero))
  (test-equal (fl- one) (flsgn negzero))
  (test one (flsgn two))
  (test-equal (fl- one) (flsgn (fl- two)))
  (test-equal (flonum 9) (flnumerator (flonum 2.25)))
  (test-equal (flonum 4) (fldenominator (flonum 2.25)))
  (test-equal (flonum -9) (flnumerator (flonum -2.25)))
  (test-equal (flonum 4) (fldenominator (flonum -2.25)))
  (test ints (map flnumerator ints))
  (test-equal (map (lambda (x) one) ints) (map fldenominator ints))
  (test weird (map flnumerator weird))
  (test-equal (list one one) (map fldenominator infinities))
  (test-assert (flnan? (flnumerator nan)))
  (test-assert (flnan? (fldenominator nan)))
  (test-equal (flonum -4) (flfloor    (flonum -3.125)))
  (test-equal (flonum -3) (flceiling  (flonum -3.125)))
  (test-equal (flonum -3) (flround    (flonum -3.125)))
  (test-equal (flonum -3) (fltruncate (flonum -3.125)))
  (test-equal (flonum -4) (flfloor    (flonum -3.75)))
  (test-equal (flonum -3) (flceiling  (flonum -3.75)))
  (test-equal (flonum -4) (flround    (flonum -3.75)))
  (test-equal (flonum -3) (fltruncate (flonum -3.75)))
  (test-equal (flonum -4) (flfloor    (flonum -3.5)))
  (test-equal (flonum -3) (flceiling  (flonum -3.5)))
  (test-equal (flonum -4) (flround    (flonum -3.5)))
  (test-equal (flonum -3) (fltruncate (flonum -3.5)))
  (test ints (map flfloor    ints))
  (test ints (map flceiling  ints))
  (test ints (map flround    ints))
  (test ints (map fltruncate ints))
  (test-equal (map (lambda (x) zero) posfracs) (map flfloor    posfracs))
  (test-equal (map (lambda (x) one) posfracs) (map flceiling  posfracs))
  (test-equal (map (lambda (x) zero) posfracs) (map flround    posfracs))
  (test-equal (map (lambda (x) zero) posfracs) (map fltruncate posfracs))
  (test weird (map flfloor    weird))
  (test weird (map flceiling  weird))
  (test weird (map flround    weird))
  (test weird (map fltruncate weird))
)

(test-group "flonum/log+exp"
  (test one (flexp negzero))
  (test one (flexp zero))
  (test fl-e (flexp one))
  (test-approx fl-1/e (flexp (fl- one)))
  (test-approx fl-e-2 (flexp two))
  (test-approx fl-e-pi/4 (flexp fl-pi/4))
  (test posinf (flexp posinf))
  (test posinf (flexp fl-greatest))
  (test-approx one (flexp fl-least))
  (test-approx zero (flexp (fl- fl-greatest)))
  (test-approx zero (flexp neginf))
  (test one (fl+ one (flexp-1 negzero)))
  (test one (fl+ one (flexp-1 zero)))
  (test fl-e (fl+ one (flexp-1 one)))
  (test-approx fl-1/e (fl+ one (flexp-1 (fl- one))))
  (test-approx fl-e-2 (fl+ one (flexp-1 two)))
  (test-approx fl-e-pi/4 (fl+ one (flexp-1 fl-pi/4)))
  (test posinf (fl+ one (flexp-1 posinf)))
  (test posinf (fl+ one (flexp-1 fl-greatest)))
  (test-approx one (fl+ one (flexp-1 fl-least)))
  (test-approx zero (fl+ one (flexp-1 (fl- fl-greatest))))
  (test-approx zero (fl+ one (flexp-1 neginf)))
  (test one (flexp2 negzero))
  (test one (flexp2 zero))
  (test two (flexp2 one))
  (test (fl/ two) (flexp2 (fl- one)))
  (test (fl* two two) (flexp2 two))
  (test-approx fl-e (flexp2 fl-log2-e))
  (test-approx fl-e (flexp2 fl-log2-e))
  (test posinf (flexp2 posinf))
  (test posinf (flexp2 fl-greatest))
  (test-approx one (flexp2 fl-least))
  (test-approx zero (flexp2 (fl- fl-greatest)))
  (test-approx zero (flexp2 neginf))
  (test zero (flsquare zero))
  (test one (flsquare one))
  (test-equal (fl+ two two) (flsquare two))
  (test-approx two (flsquare fl-sqrt-2))
  (test-approx (flonum 3) (flsquare fl-sqrt-3))
  (test-approx (flonum 5) (flsquare fl-sqrt-5))
  (test-approx (flonum 10) (flsquare fl-sqrt-10))
  (test-equal (flonum 25) (flsquare (flonum -5)))
  (test posinf (flsquare neginf))
  (test posinf (flsquare posinf))
  (test zero (flsqrt zero))
  (test one (flsqrt one))
  (test-approx fl-sqrt-2 (flsqrt two))
  (test-approx fl-sqrt-3 (flsqrt (flonum 3)))
  (test-approx fl-sqrt-5 (flsqrt (flonum 5)))
  (test-approx fl-sqrt-10 (flsqrt (flonum 10)))
  (test-approx (flonum 26.419689627245813) (flsqrt (flonum 698)))
  (test posinf (flsqrt posinf))
  (test zero (flcbrt zero))
  (test one (flcbrt one))
  (test-approx (flcbrt two) fl-cbrt-2)
  (test-approx (flcbrt (flonum 3)) fl-cbrt-3)
  (test-approx (flcbrt (flonum 698)) (flonum 8.8705757224791313))
  (test-approx (flcbrt (flonum 11.390625)) (flonum 2.25))
  (test-approx (flcbrt (flonum -11.390625)) (flonum -2.25))
  (test posinf (flcbrt posinf))
  (test neginf (flcbrt neginf))
  (test zero (flhypot zero zero))
  (test one (flhypot zero one))
  (test-approx (flhypot two one) fl-sqrt-5)
  (test-approx (flhypot (fl- two) one) fl-sqrt-5)
  (test-approx (flhypot two (fl- one)) fl-sqrt-5)
  (test-approx (flhypot (fl- two) (fl- one)) fl-sqrt-5)
  (test-approx (flhypot (fl/ fl-greatest two) (fl/ fl-greatest two))
               (fl/ fl-greatest fl-sqrt-2))
  (test posinf (flhypot zero posinf))
  (test posinf (flhypot neginf zero))
  (test one (flexpt two zero))
  (test two (flexpt two one))
  (test-equal (flonum 4) (flexpt two two))
  (test-approx (flexpt two (fl/ two)) fl-sqrt-2)
  (test-approx (flexpt (flonum 441) (flonum 10))
               (flonum 2.7821842944695155e26))
  (test-approx (flexpt (flonum 441) (fl/ (flonum 5)))
               (flonum 3.37977444523542851))
  (for-each (lambda (x)
              (for-each (lambda (frac)
                          (test-approx (flexpt (flexpt x frac) (fl/ frac)) x))
                        posfracs))
            (filter flpositive? somereals))
  (test neginf (fllog zero))
  (test zero (fllog one))
  (test-approx (fllog two) fl-log-2)
  (test-approx (fllog (flonum 3)) fl-log-3)
  (test-approx (fllog fl-pi) fl-log-pi)
  (test-approx (fllog (flonum 10)) fl-log-10)
  (test posinf (fllog posinf))
  (for-each (lambda (x) (test-approx x (flexp (fllog x))))
            (filter flpositive? somereals))
  (test neginf (fllog2 zero))
  (test zero (fllog2 one))
  (test one (fllog2 two))
  (test-approx fl-log2-e (fllog2 fl-e))
  (test posinf (fllog2 posinf))
  (for-each (lambda (x) (test-approx (flexpt two (fllog2 x)) x))
            (filter flpositive? somereals))
  (test neginf (fllog10 zero))
  (test zero (fllog10 one))
  (test-approx (fllog10 fl-e) fl-log10-e)
  (test one (fllog10 (flonum 10)))
  (test posinf (fllog10 posinf))
  (for-each (lambda (x) (test-approx x (flexpt (flonum 10) (fllog10 x))))
            (filter flpositive? somereals))
  (test-assert (flpositive? (fllog1+ fl-least)))
  (test neginf (fllog1+ (fl- zero one)))
  (test zero (fllog1+ (fl- one one)))
  (test-approx fl-log-2 (fllog1+ (fl- two one)))
  (test-approx fl-log-3 (fllog1+ (fl- (flonum 3) one)))
  (test-approx fl-log-pi (fllog1+ (fl- fl-pi one)))
  (test-approx fl-log-10 (fllog1+ (fl- (flonum 10) one)))
  (test posinf (fllog1+ (fl- posinf one)))
  (for-each (lambda (x) (test-approx x (flexp (fllog1+ (fl- x one)))))
            (filter flpositive? somereals))
  (test-approx fl-log2-e ((make-fllog-base two) fl-e))
  (test-approx fl-log10-e ((make-fllog-base (flonum 10)) fl-e))
  (test-approx fl-log-2 ((make-fllog-base fl-e) two))
  (for-each (lambda (base)
              (let ((f (make-fllog-base base)))
                (for-each (lambda (x) (test-approx x (flexpt (flonum base) (f x))))
                          (filter flpositive? somereals))))
            (map flonum '(3 7 19)))
)

(test-group "flonum/trigonometric"
  (test-approx (flsin zero)           zero)
  (test-approx (flcos zero)           one)
  (test-approx (fltan zero)           zero)
  (test-approx (flsin (flonum 0.2))   0.19866933079506121545941)
  (test-approx (flcos (flonum 0.2))   0.98006657784124163112420)
  (test-approx (flsin (flonum 0.5))   0.47942553860420300027329)
  (test-approx (flcos (flonum 0.5))   0.87758256189037271611628)
  (test-approx (flsin (flonum 0.7))   0.64421768723769105367261)
  (test-approx (flcos (flonum 0.7))   0.76484218728448842625586)
  (test-approx (flsin fl-pi/4)        fl-1/sqrt-2)
  (test-approx (flcos fl-pi/4)        fl-1/sqrt-2)
  (test-approx (flsin one)            0.84147098480789651665250)
  (test-approx (flcos one)            0.54030230586813971740094)
  (test-approx (flsin fl-pi/2)        one)
  (test-approx (flcos fl-pi/2)        zero)
  (test-approx (flsin two)            0.90929742682568169539602)
  (test-approx (flcos two)            -0.41614683654714238699757)
  (test-approx (flsin (flonum 3))     0.14112000805986722210074)
  (test-approx (flcos (flonum 3))     -0.98999249660044545727157)
  (test-approx (flsin fl-pi)          zero)
  (test-approx (flcos fl-pi)          (fl- one))
  (test-approx (flsin fl-2pi)         zero)
  (test-approx (flcos fl-2pi)         one)
  (test-approx (flsin (flonum 35))    -0.42818266949615100440675)
  (test-approx (flcos (flonum 35))    -0.90369220509150675984730)
  (for-each (lambda (x)
              (test-approx (fl- (flsin (fl- x))) (flsin x))
              (test-approx (flcos (fl- fl-2pi x)) (flcos x))
              (test-approx (fl/ (flsin x) (flcos x)) (fltan x))
              (test-approx one (flhypot (flsin x) (flcos x))))
            (filter (lambda (x)
                      (and (flnormalized? x) (fl<? (flabs x) (flonum 10000))))
                    (append somereals posfracs)))
  (for-each (lambda (x)
              (test-approx x (flsin (flasin x))))
              ; (test-approx x (flcos (flacos x))))
            (filter (lambda (x) (fl<=? (fl- one) x one))
                    (append somereals posfracs)))
  (let ((xs (filter (lambda (x)
                      (and (flnormalized? x) (fl<? (flabs x) (flonum 10000))))
                    (append somereals posfracs))))
    (for-each (lambda (x)
                (let ((theta (flatan x)))
                  (test-approx x (fl/ (flsin theta) (flcos theta))))
                (for-each (lambda (y)
                            (let ((theta (flatan x)))
                              (test-approx (flatan (flsin theta)
                                                   (flcos theta))
                                           theta)))
                          xs))
              xs))
  (test-approx zero (flsinh zero))
  (test-approx one (flcosh zero))
  (test-approx 0.201336002541094 (flsinh (flonum 0.2)))
  (test-approx 1.020066755619076 (flcosh (flonum 0.2)))
  (test-approx 0.521095305493747 (flsinh (flonum 0.5)))
  (test-approx 1.127625965206381 (flcosh (flonum 0.5)))
  (test-approx 0.758583701839534 (flsinh (flonum 0.7)))
  (test-approx 1.255169005630943 (flcosh (flonum 0.7)))
  (test-approx 1.175201193643801 (flsinh one))
  (test-approx 1.543080634815244 (flcosh one))
  (test-approx 3.626860407847019 (flsinh two))
  (test-approx 3.762195691083631 (flcosh two))
  (test-approx 10.01787492740990 (flsinh (flonum 3)))
  (test-approx 10.06766199577777 (flcosh (flonum 3)))
  (test-approx 11013.23287470339 (flsinh (flonum 10)))
  (test-approx 11013.23292010332 (flcosh (flonum 10)))
  (for-each (lambda (x)
              (test-approx x (flasinh (flsinh x)))
              (test-approx (flabs x) (flacosh (flcosh x)))
              (test-approx x (flatanh (fltanh x))))
            (filter (lambda (x) (fl<? (flabs x) (flonum 100)))
                    (append somereals posfracs (map fl- posfracs))))
)

(test-end)
