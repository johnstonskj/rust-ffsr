;;; Oblique projection of points, lines and planes
;;; 
;;; This program visualizes points, lines and planes in a 3-dimensional cartesian coordinate
;;; system using an oblique projection as described in http://zenger.org/papers/fa.pdf (in
;;; German) and displays the visualizations in a LispPad graphics window. All projection and
;;; transformation logic is contained in the LispKit example code `VisualizePointSets.scm`.
;;; The code in this program simply displays one or two visualizations in a window.
;;; 
;;; Example usage:
;;;   (show-projection point-set-1 "Point set 1")
;;;   (show-projection point-set-2 "Point set 2")
;;;   (show-dual-projection point-set-1 point-set-2)
;;; 
;;; Author: Matthias Zenger
;;; Copyright © 2021 Matthias Zenger. All rights reserved.
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

(import (lisppad system))

;; This loads the domain logic and includes all the necessary imports
(load "Examples/VisualizePointSets")

;; Procedures for displaying point set visualizations in LispPad graphics windows

(define (show-projection ps . args)
  (let-optionals args ((title "Projection view")
                       (dsize (size 400 400))
                       (proj (current-projection)))
    (let* ((d      (make-drawing))
           (wsize  (size (size-width dsize) (+ (size-height dsize) 51)))
           (window (use-graphics-window d dsize title #f wsize)))
      (draw-projection proj ps d)
      (set-graphics-window-label! window "Made with LispPad")
      window)))

(define (show-dual-projection ps1 ps2 . args)
  (let-optionals args ((title "Dual projection view")
                       (dsize (size 800 400))
                       (proj (current-projection)))
    (let* ((d1     (make-drawing))
           (d2     (make-drawing))
           (wsize  (size (size-width dsize) (+ (size-height dsize) 51)))
           (shift  (translate (fl/ (size-width dsize) 2.0) 0))
           (window (use-graphics-window d1 dsize title #f wsize)))
      (draw-projection proj ps1 d1)
      (draw-projection proj ps2 d2)
      (set-color black d1)
      (enable-transformation shift d1)
      (draw-drawing d2 d1)
      (disable-transformation shift d1)
      (set-graphics-window-label! window "Made with LispPad")
      window)))

;; Demo point sets

(define point-set-1 (list (list (plane #(0 0 0) #(1.0 0.1 -0.5) #(0.0 0.8 0.8))
                                (color 0.6 0.6 0.6) (color 0 0 1 0.15))
                          (list (plane #(0 -7.5 0) #(1.0 0.0 0.0) #(0.0 0.0 1.0))
                                (color 0.6 0.6 0.6) (color 1 0 0 0.15))))

(define point-set-2 (list (plane #(0 0 0) #(0.0 0.0 1.0) #(1.0 0.0 0.0))
                          (plane #(0 0 0) #(0.0 0.0 1.0) #(1.7 1.0 0.0))
                          (plane #(0 0 0) #(0.0 0.0 1.0) #(1.0 1.7 0.0))
                          (plane #(0 0 0) #(0.0 0.0 1.0) #(0.0 1.0 0.0))
                          (plane #(0 0 0) #(0.0 0.0 1.0) #(-1.0 1.7 0.0))
                          (plane #(0 0 0) #(0.0 0.0 1.0) #(1.7 -1.0 0.0))))
