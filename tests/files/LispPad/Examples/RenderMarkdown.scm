;;; Render markdown in a graphics window
;;;
;;; This is a demo showing how to combine functionality to parse and convert text in
;;; Markdown format with LispKit's drawing library as well as LispPad's capabilities to
;;; create graphics windows and display rich text.
;;;
;;; Function `render-markdown` below reads a text file at _path_, parses it as text
;;; in Markdown format, converts that into HTML and displays the HTML in a new graphics
;;; window with _title_ as its title. The function returns the identity of the window.
;;;
;;; Author: Matthias Zenger
;;; Copyright © 2020 Matthias Zenger. All rights reserved.
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
        (lispkit markdown)
        (lispkit draw)
        (lisppad system))

(define (render-markdown path title . stylespec)
  (let* ((md (markdown (read-file path)))
         (html (apply markdown->html-doc md stylespec))
         (dsize (html-size html 600.0))
         (wsize (size 616.0 600.0))
         (rendering (drawing (draw-html html (rect (point 8 4) dsize)))))
    (use-graphics-window rendering dsize title #f wsize)))

(render-markdown (asset-file-path "LispKitDatatype" "md" "Documents")
                 "LispKit Datatype"
                 '(14.0 "Helvetica" "#000000")
                 '(14.0 "Courier" "#333399")
                 '(13.0 "#333399" "#F5F5FF")
                 '("#D0D0EE" #f "#000099" "#0000CC"))
