;;; Simple drawing displayed in a new graphics window
;;;
;;; This is a demo showing how to create drawings and how to display them in a new
;;; graphics window.
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

;; Create a new empty drawing
(define d (make-drawing))

;; Draw circles and fill them in different colors
(with-drawing d
  (set-fill-color black)
  (fill (circle (point 50 50) 40))
  (set-fill-color red)
  (fill (circle (point 150 50) 40))
  (set-fill-color gray)
  (fill (circle (point 250 50) 40))
  (set-fill-color green)
  (fill (circle (point 100 150) 40))
  (set-fill-color blue)
  (fill (circle (point 200 150) 40))
  (set-fill-color yellow)
  (fill (circle (point 300 150) 40))
  (set-fill-color yellow)
  (fill (circle (point 150 250) 40))
  (draw (circle (point 150 250) 40) 1)
  (set-color black)
  (draw (circle (point 250 250) 40) 1)
  (set-fill-color red)
  (fill (circle (point 350 250) 40))
  (draw (circle (point 350 250) 40) 1)
  (set-fill-color red)
  (fill (circle (point 200 350) 40))
  (draw (circle (point 200 350) 40) 2)
  (set-color black)
  (draw (circle (point 300 350) 40) 2)
  (set-fill-color green)
  (fill (circle (point 400 350) 40))
  (draw (circle (point 400 350) 40) 2))

;; Show the drawing in a new graphics window
(make-graphics-window d (size 450 400) "Simple Graphics" (point 50 50) (size 450 450))
