;;; LISPKIT LOGIC
;;;
;;; This library provides an implementation of miniKanren. miniKanren is an embedded
;;; domain-specific language for logic programming. The core miniKanren language is simple,
;;; with only three logical operators and one interface operator. The core language is
;;; described in this short, interactive tutorial:
;;; http://io.livecode.ch/learn/webyrd/webmk
;;;
;;; The book "The Reasoned Schemer" by Daniel Friedman, William Byrd, and Oleg Kiselyov
;;; provides an in-depth introduction and tutorial for all aspects of miniKanren.
;;;
;;; The code below is based on the implementation of miniKanren with symbolic constraints
;;; by Jason Hemann, William Byrd, and Dan Friedman. The original code can be found here:
;;; https://github.com/webyrd/miniKanren-with-symbolic-constraints
;;;
;;;   November 28, 2014:
;;;     * Fixed missing unquote before E in 'drop-Y-b/c-dup-var'
;;;     * Updated 'rem-xx-from-d' to check against other constraints after
;;;       unification, in order to remove redundant disequality constraints
;;;       subsumed by absento constraints.
;;;
;;;   September 18, 2013:
;;;     * Written by Jason Hemann, Will Byrd, and Dan Friedman
;;;     * Support for eigens:
;;;       E = (e* . x*)*, where e* is a list of eigens and x* is a list of variables.
;;;       Each e in e* is checked for any of its eigens be in any of its x*.  Then it fails.
;;;       Since eigen-occurs-check is chasing variables, we might as will do a memq instead
;;;       of an eq? when an eigen is found through a chain of walks.  See eigen-occurs-check.
;;;       All the e* must be the eigens created as part of a single eigen.  The reifier just
;;;       abandons E, if it succeeds.  If there is no failure by then, there were no eigen
;;;       violations.
;;;
;;; The MIT License (MIT)
;;; Copyright (c) 2015 William E. Byrd
;;;
;;; Permission is hereby granted, free of charge, to any person obtaining a copy
;;; of this software and associated documentation files (the "Software"), to deal
;;; in the Software without restriction, including without limitation the rights
;;; to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
;;; copies of the Software, and to permit persons to whom the Software is
;;; furnished to do so, subject to the following conditions:
;;;
;;; The above copyright notice and this permission notice shall be included in all
;;; copies or substantial portions of the Software.
;;;
;;; THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
;;; IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
;;; FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
;;; AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
;;; LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
;;; OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
;;; SOFTWARE.
;;;
;;; Adaptation to LispKit
;;;   Copyright ?? 2018 Matthias Zenger. All rights reserved.

(define-library (lispkit logic)

  (export ==
          =/=
          succeed
          fail
          fresh
          run
          run*
          conde
          conda
          ifa
          condu
          ifu
          absento
          symbolo
          numbero)

  (import (lispkit base))

  (begin

    (define (exists p l)
      (if (null? l)
          #f
          (let ((res (p (car l))))
            (if (null? (cdr l))
                res
                (if res res (exists p (cdr l)))))))

    (define (find p l)
      (if (null? l)
          #f
          (if (p (car l))
              (car l)
              (find p (cdr l)))))

    (define (remp p l)
      (if (null? l)
          '()
          (if (p (car l))
              (remp p (cdr l))
              (cons (car l) (remp p (cdr l))))))

    (define (for-all p l)
      (if (null? l)
          #t
          (let ((res (p (car l))))
            (if (null? (cdr l))
                res
                (if res (for-all p (cdr l)) #f))))))

  (begin

    (define empty-c '(() () () () () () ()))

    (define eigen-tag (box 'eigen-tag))

    (define-syntax inc
      (syntax-rules ()
        ((_ e) (lambdaf@ () e))))

    (define-syntax lambdaf@
      (syntax-rules ()
        ((_ () e) (lambda () e))))

    (define-syntax lambdag@
      (syntax-rules (:)
        ((_ (c) e)
          (lambda (c) e))
        ((_ (c : B E S) e)
          (lambda (c)
            (let ((B (c->B c))
                  (E (c->E c))
                  (S (c->S c)))
              e)))
        ((_ (c : B E S D Y N T) e)
          (lambda (c)
            (let ((B (c->B c))
                  (E (c->E c))
                  (S (c->S c))
                  (D (c->D c))
     	            (Y (c->Y c))
     	            (N (c->N c))
     	            (T (c->T c)))
              e)))))

    (define rhs cdr)

    (define lhs car)

    (define (eigen-var) (box eigen-tag))

    (define (eigen? x)
      (and (box? x) (eq? (unbox x) eigen-tag)))

    (define var box)

    (define (var? x)
      (and (box? x) (not (eq? (unbox x) eigen-tag))))

    (define (walk u S)
      (cond ((and (var? u) (assq u S)) => (lambda (pr) (walk (rhs pr) S)))
            (else u)))

    (define (prefix-S S+ S)
      (cond ((eq? S+ S) '())
            (else (cons (car S+) (prefix-S (cdr S+) S)))))

    (define (unify u v s)
      (let ((u (walk u s))
            (v (walk v s)))
        (cond ((eq? u v)                  s)
              ((var? u)                   (ext-s-check u v s))
              ((var? v)                   (ext-s-check v u s))
              ((and (pair? u) (pair? v))  (let ((s (unify (car u) (car v) s)))
                                            (and s (unify (cdr u) (cdr v) s))))
              ((or (eigen? u) (eigen? v)) #f)
              ((equal? u v)               s)
              (else                       #f))))

    (define (occurs-check x v s)
      (let ((v (walk v s)))
        (cond ((var? v)  (eq? v x))
              ((pair? v) (or (occurs-check x (car v) s) (occurs-check x (cdr v) s)))
              (else      #f))))

    (define (eigen-occurs-check e* x s)
      (let ((x (walk x s)))
        (cond ((var? x)   #f)
              ((eigen? x) (memq x e*))
              ((pair? x)  (or (eigen-occurs-check e* (car x) s) (eigen-occurs-check e* (cdr x) s)))
              (else       #f))))

    (define empty-f
      (lambdaf@ () (mzero)))

    (define (ext-s-check x v s)
      (if (occurs-check x v s) #f (cons (cons x v) s)))

    (define (unify* S+ S)
      (unify (map lhs S+) (map rhs S+) S))

    (define-syntax case-inf
      (syntax-rules ()
        ((_ e (() e0) ((f^) e1) ((c^) e2) ((c f) e3))
         (let ((c-inf e))
           (cond
             ((not c-inf) e0)
             ((procedure? c-inf)  (let ((f^ c-inf)) e1))
             ((not (and (pair? c-inf)
                     (procedure? (cdr c-inf))))
              (let ((c^ c-inf)) e2))
             (else (let ((c (car c-inf)) (f (cdr c-inf)))
                     e3)))))))

    (define-syntax fresh
      (syntax-rules ()
        ((_ (x ...) g0 g ...)
          (lambdag@ (c : B E S D Y N T)
            (inc
              (let ((x (var 'x)) ...)
                (let ((B (append (list x ...) B)))
                  (bind* (g0 (list B E S D Y N T)) g ...))))))))

    (define-syntax eigen
      (syntax-rules ()
        ((_ (x ...) g0 g ...)
          (lambdag@ (c : B E S)
            (let ((x (eigen-var)) ...)
              ((fresh () (eigen-absento (list x ...) B) g0 g ...) c))))))

    (define-syntax bind*
      (syntax-rules ()
        ((_ e) e)
        ((_ e g0 g ...) (bind* (bind e g0) g ...))))

    (define (bind c-inf g)
      (case-inf c-inf
        (() (mzero))
        ((f) (inc (bind (f) g)))
        ((c) (g c))
        ((c f) (mplus (g c) (lambdaf@ () (bind (f) g))))))

    (define-syntax run
      (syntax-rules ()
        ((_ n (q) g0 g ...)
          (take n
            (lambdaf@ ()
              ((fresh (q) g0 g ...
                 (lambdag@ (final-c)
                   (let ((z ((reify q) final-c)))
                     (choice z empty-f))))
               empty-c))))
        ((_ n (q0 q1 q ...) g0 g ...)
          (run n (x) (fresh (q0 q1 q ...) g0 g ... (== (list q0 q1 q ...) x))))))

    (define-syntax run*
      (syntax-rules ()
        ((_ (q0 q ...) g0 g ...) (run #f (q0 q ...) g0 g ...))))

    (define (take n f)
      (if (and n (zero? n))
          '()
          (case-inf (f)
            (()    '())
            ((f)   (take n f))
            ((c)   (cons c '()))
            ((c f) (cons c (take (and n (- n 1)) f))))))

    (define-syntax conde
      (syntax-rules ()
        ((_ (g0 g ...) (g1 g^ ...) ...)
          (lambdag@ (c)
            (inc
              (mplus* (bind* (g0 c) g ...) (bind* (g1 c) g^ ...) ...))))))

    (define-syntax mplus*
      (syntax-rules ()
        ((_ e) e)
        ((_ e0 e ...) (mplus e0 (lambdaf@ () (mplus* e ...))))))

    (define mplus
      (lambda (c-inf f)
        (case-inf c-inf
          (()     (f))
          ((f^)   (inc (mplus (f) f^)))
          ((c)    (choice c f))
          ((c f^) (choice c (lambdaf@ () (mplus (f) f^)))))))

    (define c->B car)
    (define c->E cadr)
    (define c->S caddr)
    (define c->D cadddr)
    (define (c->Y c) (cadddr (cdr c)))
    (define (c->N c) (cadddr (cddr c)))
    (define (c->T c) (cadddr (cdddr c)))

    (define-syntax conda
      (syntax-rules ()
        ((_ (g0 g ...) (g1 g^ ...) ...)
         (lambdag@ (c)
           (inc
             (ifa ((g0 c) g ...)
                  ((g1 c) g^ ...) ...))))))

    (define-syntax ifa
      (syntax-rules ()
        ((_) (mzero))
        ((_ (e g ...) b ...)
         (let loop ((c-inf e))
           (case-inf c-inf
             (() (ifa b ...))
             ((f) (inc (loop (f))))
             ((a) (bind* c-inf g ...))
             ((a f) (bind* c-inf g ...)))))))

    (define-syntax condu
      (syntax-rules ()
        ((_ (g0 g ...) (g1 g^ ...) ...)
         (lambdag@ (c)
           (inc
             (ifu ((g0 c) g ...)
                  ((g1 c) g^ ...) ...))))))

    (define-syntax ifu
      (syntax-rules ()
        ((_) (mzero))
        ((_ (e g ...) b ...)
         (let loop ((c-inf e))
           (case-inf c-inf
             (() (ifu b ...))
             ((f) (inc (loop (f))))
             ((c) (bind* c-inf g ...))
             ((c f) (bind* (unit c) g ...)))))))

    (define (mzero) #f)

    (define unit identity)

    (define choice cons)

    (define (tagged? S Y y^)
      (exists (lambda (y) (eqv? (walk y S) y^)) Y))

    (define (untyped-var? S Y N t^)
      (let ((in-type? (lambda (y) (eq? (walk y S) t^))))
        (and (var? t^)
             (not (exists in-type? Y))
             (not (exists in-type? N)))))

    (define-syntax project
      (syntax-rules ()
        ((_ (x ...) g g* ...)
         (lambdag@ (c : B E S)
           (let ((x (walk* x S)) ...)
             ((fresh () g g* ...) c))))))

    (define (walk* v S)
      (let ((v (walk v S)))
        (cond ((var? v)  v)
              ((pair? v) (cons (walk* (car v) S) (walk* (cdr v) S)))
              (else      v))))

    (define (reify-S v S)
      (let ((v (walk v S)))
        (cond ((var? v)
                 (let ((n (length S)))
                   (let ((name (reify-name n)))
                     (cons (cons v name) S))))
              ((pair? v)
                 (let ((S (reify-S (car v) S)))
                   (reify-S (cdr v) S)))
              (else S))))

    (define (reify-name n)
      (string->symbol (string-append "_" "." (number->string n))))

    (define (drop-dot X)
      (map (lambda (t)
             (let ((a (lhs t))
                   (d (rhs t)))
               (list a d)))
           X))

    (define (sorter ls)
      (sort lex<=? ls))

    (define (lex<=? x y)
      (string<=? (datum->string x) (datum->string y)))

    (define (datum->string x)
      (let ((port (open-output-string)))
        (display x port)
        (close-output-port port)
        (get-output-string port)))

    (define (anyvar? u r)
      (if (pair? u)
          (or (anyvar? (car u) r) (anyvar? (cdr u) r))
          (var? (walk u r))))

    (define (anyeigen? u r)
      (if (pair? u)
          (or (anyeigen? (car u) r) (anyeigen? (cdr u) r))
          (eigen? (walk u r))))

    (define (member* u v)
      (cond ((equal? u v) #t)
            ((pair? v)    (or (member* u (car v)) (member* u (cdr v))))
            (else         #f)))

    ;;;

    (define drop-N-b/c-const
      (lambdag@ (c : B E S D Y N T)
        (let ((const? (lambda (n)
                        (not (var? (walk n S))))))
          (cond ((find const? N) =>
                   (lambda (n) (list B E S D Y (remq1 n N) T)))
                (else c)))))

    (define drop-Y-b/c-const
      (lambdag@ (c : B E S D Y N T)
        (let ((const? (lambda (y)
                        (not (var? (walk y S))))))
          (cond ((find const? Y) =>
                   (lambda (y) (list B E S D (remq1 y Y) N T)))
                (else c)))))

    (define (remq1 elem ls)
      (cond ((null? ls) '())
            ((eq? (car ls) elem) (cdr ls))
            (else (cons (car ls) (remq1 elem (cdr ls))))))

    (define (same-var? v)
      (lambda (v^)
        (and (var? v) (var? v^) (eq? v v^))))

    (define (find-dup f S)
      (lambda (set)
        (let loop ((set^ set))
          (if (null? set^)
              #f
              (let* ((elem (car set^))
                     (elem^ (walk elem S)))
                (if (find (lambda (elem^^) ((f elem^) (walk elem^^ S))) (cdr set^))
                    elem
                    (loop (cdr set^))))))))

    (define drop-N-b/c-dup-var
      (lambdag@ (c : B E S D Y N T)
        (cond
          (((find-dup same-var? S) N) =>
             (lambda (n) (list B E S D Y (remq1 n N) T)))
          (else c))))

    (define drop-Y-b/c-dup-var
      (lambdag@ (c : B E S D Y N T)
        (cond
          (((find-dup same-var? S) Y) =>
             (lambda (y)
               (list B E S D (remq1 y Y) N T)))
          (else c))))

    (define (var-type-mismatch? S Y N t1^ t2^)
      (cond ((num? S N t1^) (not (num? S N t2^)))
            ((sym? S Y t1^) (not (sym? S Y t2^)))
            (else           #f)))

    (define (term-ununifiable? S Y N t1 t2)
      (let ((t1^ (walk t1 S))
            (t2^ (walk t2 S)))
        (cond ((or (untyped-var? S Y N t1^) (untyped-var? S Y N t2^)) #f)
              ((var? t1^) (var-type-mismatch? S Y N t1^ t2^))
              ((var? t2^) (var-type-mismatch? S Y N t2^ t1^))
              ((and (pair? t1^) (pair? t2^))
                 (or (term-ununifiable? S Y N (car t1^) (car t2^))
                     (term-ununifiable? S Y N (cdr t1^) (cdr t2^))))
              (else (not (eqv? t1^ t2^))))))

    (define (T-term-ununifiable? S Y N)
      (lambda (t1)
        (let ((t1^ (walk t1 S)))
          (letrec
              ((t2-check
                (lambda (t2)
                  (let ((t2^ (walk t2 S)))
                    (if (pair? t2^)
                        (and (term-ununifiable? S Y N t1^ t2^)
                             (t2-check (car t2^))
                             (t2-check (cdr t2^)))
                        (term-ununifiable? S Y N t1^ t2^))))))
            t2-check))))

    (define (num? S N n)
      (let ((n (walk n S)))
        (if (var? n)
            (tagged? S N n)
            (number? n))))

    (define (sym? S Y y)
      (let ((y (walk y S)))
        (if (var? y)
            (tagged? S Y y)
            (symbol? y))))

    (define drop-T-b/c-Y-and-N
      (lambdag@ (c : B E S D Y N T)
        (let ((drop-t? (T-term-ununifiable? S Y N)))
          (cond
            ((find (lambda (t) ((drop-t? (lhs t)) (rhs t))) T) =>
             (lambda (t) (list B E S D Y N (remq1 t T))))
            (else c)))))

    (define move-T-to-D-b/c-t2-atom
      (lambdag@ (c : B E S D Y N T)
        (cond
          ((exists (lambda (t)
                     (let ((t2^ (walk (rhs t) S)))
                       (cond
                         ((and (not (untyped-var? S Y N t2^))
                               (not (pair? t2^)))
                          (let ((T (remq1 t T)))
                            (list B E S (cons (list t) D) Y N T)))
                         (else #f))))
                     T))
          (else c))))

    (define (terms-pairwise=? pr-a^ pr-d^ t-a^ t-d^ S)
      (or (and (term=? pr-a^ t-a^ S)
               (term=? pr-d^ t-a^ S))
          (and (term=? pr-a^ t-d^ S)
               (term=? pr-d^ t-a^ S))))

    (define (T-superfluous-pr? S Y N T)
      (lambda (pr)
        (let ((pr-a^ (walk (lhs pr) S))
              (pr-d^ (walk (rhs pr) S)))
          (cond
            ((exists
                 (lambda (t)
                   (let ((t-a^ (walk (lhs t) S))
                         (t-d^ (walk (rhs t) S)))
                     (terms-pairwise=? pr-a^ pr-d^ t-a^ t-d^ S)))
               T)
             (for-all
              (lambda (t)
                (let ((t-a^ (walk (lhs t) S))
                      (t-d^ (walk (rhs t) S)))
                  (or
                   (not (terms-pairwise=? pr-a^ pr-d^ t-a^ t-d^ S))
                   (untyped-var? S Y N t-d^)
                   (pair? t-d^))))
              T))
            (else #f)))))

    (define drop-from-D-b/c-T
      (lambdag@ (c : B E S D Y N T)
        (cond
          ((find
               (lambda (d)
                 (exists
                     (T-superfluous-pr? S Y N T)
                   d))
             D) =>
             (lambda (d) (list B E S (remq1 d D) Y N T)))
          (else c))))

    (define drop-t-b/c-t2-occurs-t1
      (lambdag@ (c : B E S D Y N T)
        (cond
          ((find (lambda (t)
                   (let ((t-a^ (walk (lhs t) S))
                         (t-d^ (walk (rhs t) S)))
                     (mem-check t-d^ t-a^ S)))
                 T) =>
                 (lambda (t)
                   (list B E S D Y N (remq1 t T))))
          (else c))))

    (define split-t-move-to-d-b/c-pair
      (lambdag@ (c : B E S D Y N T)
        (cond
          ((exists
             (lambda (t)
               (let ((t2^ (walk (rhs t) S)))
                 (cond
                   ((pair? t2^) (let ((ta (cons (lhs t) (car t2^)))
                                      (td (cons (lhs t) (cdr t2^))))
                                  (let ((T `(,ta ,td . ,(remq1 t T))))
                                    (list B E S (cons (list t) D) Y N T))))
                   (else #f))))
             T))
          (else c))))

    (define (find-d-conflict S Y N)
      (lambda (D)
        (find (lambda (d) (exists (lambda (pr) (term-ununifiable? S Y N (lhs pr) (rhs pr))) d)) D)))

    (define drop-D-b/c-Y-or-N
      (lambdag@ (c : B E S D Y N T)
        (cond
          (((find-d-conflict S Y N) D) =>
           (lambda (d) (list B E S (remq1 d D) Y N T)))
          (else c))))

    (define cycle
      (lambdag@ (c)
        (let loop ((c^ c)
                   (fns^ (LOF))
                   (n (length (LOF))))
          (cond ((zero? n) c^)
                ((null? fns^) (loop c^ (LOF) n))
                (else
                  (let ((c^^ ((car fns^) c^)))
                    (if (not (eq? c^^ c^))
                        (loop c^^ (cdr fns^) (length (LOF)))
                        (loop c^ (cdr fns^) (fx1- n)))))))))

    (define (absento u v)
      (lambdag@ (c : B E S D Y N T)
        (if (mem-check u v S)
            (mzero)
            (unit (list B E S D Y N (cons (cons u v) T))))))

    (define (eigen-absento e* x*)
      (lambdag@ (c : B E S D Y N T)
        (if (eigen-occurs-check e* x* S)
            (mzero)
            (unit (list B (cons (cons e* x*) E) S D Y N T)))))

    (define (mem-check u t S)
      (let ((t (walk t S)))
        (if (pair? t)
            (or (term=? u t S)
                (mem-check u (car t) S)
                (mem-check u (cdr t) S))
            (term=? u t S))))

    (define (term=? u t S)
      (cond ((unify u t S) => (lambda (S0) (eq? S0 S)))
             (else #f)))

    (define (ground-non-<type>? pred)
      (lambda (u S)
        (let ((u (walk u S)))
          (if (var? u) #f (not (pred u))))))

    ;; moved
    (define ground-non-symbol?
      (ground-non-<type>? symbol?))

    (define ground-non-number?
      (ground-non-<type>? number?))

    (define (symbolo u)
      (lambdag@ (c : B E S D Y N T)
        (cond ((ground-non-symbol? u S) (mzero))
              ((mem-check u N S)        (mzero))
              (else                     (unit (list B E S D (cons u Y) N T))))))

    (define (numbero u)
      (lambdag@ (c : B E S D Y N T)
        (cond ((ground-non-number? u S) (mzero))
              ((mem-check u Y S)        (mzero))
              (else                     (unit (list B E S D Y (cons u N) T))))))

    ;; end moved

    (define (=/= u v)
      (lambdag@ (c : B E S D Y N T)
        (cond
          ((unify u v S) =>
           (lambda (S0)
             (let ((pfx (prefix-S S0 S)))
               (if (null? pfx)
                   (mzero)
                   (unit (list B E S (cons pfx D) Y N T))))))
          (else c))))

    (define (== u v)
      (lambdag@ (c : B E S D Y N T)
        (cond
          ((unify u v S) =>
           (lambda (S0)
             (if (==fail-check B E S0 D Y N T)
                 (mzero)
                 (unit (list B E S0 D Y N T)))))
          (else (mzero)))))

    (define succeed (== #f #f))

    (define fail (== #f #t))

    (define (==fail-check B E S0 D Y N T)
      (cond ((eigen-absento-fail-check E S0) #t)
            ((atomic-fail-check S0 Y ground-non-symbol?) #t)
            ((atomic-fail-check S0 N ground-non-number?) #t)
            ((symbolo-numbero-fail-check S0 Y N) #t)
            ((=/=-fail-check S0 D) #t)
            ((absento-fail-check S0 T) #t)
            (else #f)))

    (define (eigen-absento-fail-check E S0)
      (exists (lambda (e*/x*) (eigen-occurs-check (car e*/x*) (cdr e*/x*) S0)) E))

    (define (atomic-fail-check S A pred)
      (exists (lambda (a) (pred (walk a S) S)) A))

    (define (symbolo-numbero-fail-check S A N)
      (let ((N (map (lambda (n) (walk n S)) N)))
        (exists (lambda (a) (exists (same-var? (walk a S)) N)) A)))

    (define (absento-fail-check S T)
      (exists (lambda (t) (mem-check (lhs t) (rhs t) S)) T))

    (define (=/=-fail-check S D)
      (exists (d-fail-check S) D))

    (define (d-fail-check S)
      (lambda (d) (cond ((unify* d S) => (lambda (S+) (eq? S+ S)))
                        (else #f))))

    (define (reify x)
      (lambda (c)
        (let ((c (cycle c)))
          (let* ((S (c->S c))
                 (D (walk* (c->D c) S))
                 (Y (walk* (c->Y c) S))
                 (N (walk* (c->N c) S))
                 (T (walk* (c->T c) S)))
          (let ((v (walk* x S)))
            (let ((R (reify-S v '())))
              (reify+ v R
                      (let ((D (remp
                                (lambda (d)
                                  (let ((dw (walk* d S)))
                                    (or
                                      (anyvar? dw R)
                                      (anyeigen? dw R))))
                                 (rem-xx-from-d c))))
                        (rem-subsumed D))
                      (remp (lambda (y) (var? (walk y R))) Y)
                      (remp (lambda (n) (var? (walk n R))) N)
                      (remp (lambda (t) (or (anyeigen? t R) (anyvar? t R))) T))))))))

    (define (reify+ v R D Y N T)
      (form (walk* v R)
            (walk* D R)
            (walk* Y R)
            (walk* N R)
            (rem-subsumed-T (walk* T R))))

    (define (form v D Y N T)
      (let ((fd (sort-D D))
            (fy (sorter Y))
            (fn (sorter N))
            (ft (sorter T)))
        (let ((fd (if (null? fd) fd
                      (let ((fd (drop-dot-D fd)))
                        `((=/= . ,fd)))))
              (fy (if (null? fy) fy `((sym . ,fy))))
              (fn (if (null? fn) fn `((num . ,fn))))
              (ft (if (null? ft) ft
                      (let ((ft (drop-dot ft)))
                        `((absento . ,ft))))))
          (if (and (null? fd) (null? fy) (null? fn) (null? ft))
              v
              (append (list v) fd fn fy ft)))))

    (define (sort-D D)
      (sorter (map sort-d D)))

    (define (sort-d d)
      (sort (lambda (x y) (lex<=? (car x) (car y))) (map sort-pr d)))

    (define (drop-dot-D D)
      (map drop-dot D))

    (define (lex<-reified-name? r)
      (char<? (string-ref (datum->string r) 0) #\_))

    (define (sort-pr pr)
      (let ((l (lhs pr))
            (r (rhs pr)))
        (cond ((lex<-reified-name? r) pr)
              ((lex<=? r l)           (cons r l))
              (else                   pr))))

    (define (rem-subsumed D)
      (let rem-subsumed ((D D) (d^* '()))
        (cond
          ((null? D)
            d^*)
          ((or (subsumed? (car D) (cdr D))
               (subsumed? (car D) d^*))
            (rem-subsumed (cdr D) d^*))
          (else
            (rem-subsumed (cdr D) (cons (car D) d^*))))))

    (define (subsumed? d d*)
      (if (null? d*)
          #f
          (let ((d^ (unify* (car d*) d)))
            (or (and d^ (eq? d^ d)) (subsumed? d (cdr d*))))))

    (define rem-xx-from-d
      (lambdag@ (c : B E S D Y N T)
        (let ((D (walk* D S)))
          (remp not
                (map (lambda (d)
                       (cond
                         ((unify* d S) =>
                            (lambda (S0)
                              (if (==fail-check B E S0 '() Y N T) #f (prefix-S S0 S))))
                         (else #f)))
                     D)))))

    (define (rem-subsumed-T T)
      (let rem-subsumed ((T T) (T^ '()))
        (if (null? T)
            T^
            (let ((lit (lhs (car T)))
                  (big (rhs (car T))))
              (if (or (subsumed-T? lit big (cdr T)) (subsumed-T? lit big T^))
                  (rem-subsumed (cdr T) T^)
                  (rem-subsumed (cdr T) (cons (car T) T^)))))))

    (define (subsumed-T? lit big T)
      (if (null? T)
          #f
          (let ((lit^ (lhs (car T)))
                (big^ (rhs (car T))))
            (or (and (eq? big big^) (member* lit^ lit))
                (subsumed-T? lit big (cdr T))))))

    (define (LOF)
      (list drop-N-b/c-const
            drop-Y-b/c-const
            drop-Y-b/c-dup-var
            drop-N-b/c-dup-var
            drop-D-b/c-Y-or-N
            drop-T-b/c-Y-and-N
            move-T-to-D-b/c-t2-atom
            split-t-move-to-d-b/c-pair
            drop-from-D-b/c-T
            drop-t-b/c-t2-occurs-t1))
  )
)
