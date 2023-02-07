;;; Plot graphs in a LispPad graphics window
;;;
;;; This is a demo of the libraries `(lispkit draw)` and `(lisppad system)`. Function `plot`
;;; draws a graph of a function for a given range using a number of interpolation points.
;;; Function `show-graph-plot` shows a plotted drawing in a graphics window.
;;;
;;; Usage: (show-graph-plot sin -1 6.5 50 "sin(x)")
;;;        (show-graph-plot (lambda (x) (* (sin (* x 2)) (cos (/ x 4)))) -1 6.5 50 "sin+cos")
;;;
;;; Author: Matthias Zenger
;;; Copyright © 2018 Matthias Zenger. All rights reserved.
;;;
;;; Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file
;;; except in compliance with the License. You may obtain a copy of the License at
;;;
;;;   http://www.apache.org/licenses/LICENSE-2.0
;;;
;;; Unless required by applicable law or agreed to in writing, software distributed under the
;;; License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND,
;;; either express or implied. See the License for the specific language governing permissions
;;; and limitations under the License.

(import (lispkit base)
        (lispkit draw)
        (lisppad system))

;; Plots a function `f` over range `[xmin; xmax]` using `n` interpolation points
;; within rectangle `rect`. Prints `label` at the bottom of the graph. Returns the result
;; as a drawing object.
(define (plot f xmin xmax n rect)
  (let* ((dx (/ (- xmax xmin) n))
         (xs (tabulate (fx1+ n) (lambda (i) (+ xmin (* i dx)))))
         (ys (map f xs))
         (ymin (apply min ys))
         (ymax (apply max ys))
         (xfac (/ (rect-width rect) (- xmax xmin)))
         (yfac (/ (rect-height rect) (- ymax ymin)))
         (ps (map (lambda (x y) (point (* xfac (- x xmin)) (* yfac (- y ymin)))) xs ys))
         (graph (flip-shape (interpolate ps))))
    (drawing
      ; Draw a bounding box
      (draw (rectangle (rect-point rect) (rect-size rect)) 1.0)
      ; Move rest of drawing into the bounding box
      (transform (translate (rect-x rect) (rect-y rect))
        ; Draw the coordinate axis
        (set-color (color 0.3 0.3 0.3))
        (if (and (<= xmin 0.0) (>= xmax 0.0))
          (draw (line (point (* xfac (- xmin)) 0)
                      (point (* xfac (- xmin)) (rect-height rect))) 0.5))
        (if (and (<= ymin 0.0) (>= ymax 0.0))
          (draw (line (point 0 (+ (rect-height rect) (* yfac ymin)))
                      (point (rect-width rect) (+ (rect-height rect) (* yfac ymin)))) 0.5))
        ; Draw flipped interpolation shape
        (set-color blue)
        (draw graph 1.5)
        ; Draw interpolation points
        (set-fill-color white)
        (for-each (lambda (p)
                    (let ((s (flip-shape (circle p 2.5) (shape-bounds graph))))
                      (fill s) (draw s 1.0))) ps)))))

;; Creates a demo page consisting of a header and four graphs
(define (show-graph-plot f xmin xmax n title)
  (let* ((graph (drawing
                  (draw-text "Demo of libraries (lispkit draw) and (lisppad system)"
                             (point 110 16)
                             (font "Times-Bold" 16)
                             black)
                  (draw-drawing (plot f xmin xmax n (rect 20 50 510 330)))))
         (window (use-graphics-window graph (size 550 400) title #f (size 550 445))))
    (set-graphics-window-label! window "© 2018 Matthias Zenger")
    window))
