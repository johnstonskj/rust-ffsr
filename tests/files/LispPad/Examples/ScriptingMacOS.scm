;;; Integrate with other macOS applications via AppleScript
;;;
;;; This example shows how to invoke AppleScript sub-routines from LispPad. Since
;;; LispPad runs in a sandbox and scripts are executed outside of the sandbox, this
;;; will allow direct integrations with other macOS applications such as Mail, Safari,
;;; Music, etc.
;;;
;;; The script authorization mechanism for users is unfortunately a bit cumbersome, requiring
;;; the installation of the AppleScript files in a particular directory specifically
;;; for LispPad. `(system-directory 'application-scripts)` returns a list of directories
;;; in which AppleScripts are accessible by LispPad. It includes typically the directory:
;;; /Users/username/Library/Application Scripts/net.objecthub.LispPad
;;;
;;; The directory can be opened on macOS's Finder via:
;;;   (open-file (car (system-directory 'application-scripts)))
;;;
;;; For this example code to work, the "Sample.scpt" script that is included with LispPad
;;; needs to be installed on the system. This can be done with the following procedure call.
;;; You need to click the "Save script" button to store the script in the right directly:
;;;   (install-sample-script #t)
;;;
;;; To verify that the script is installed correctly, you can look at it with the script editor:
;;;   (view-script sample-script-file)
;;;
;;; It is now possible to define an `applescript` object for this script via library
;;; `(lisppad applescript)`. With this object, it will be possible to invoke the script
;;; as a whole and call individual sub-routines of it (the major use case).
;;; Library `(lisppad applescript)` can also invoke Unix scripts and Automator workflows.
;;;
;;; Here is how to invoke the AppleScript sub-routine `safariFrontURL` of file "Sample.scpt"
;;; in the application scripts folder. This sub-routine has no arguments and returns the URL
;;; of the front window of Safari:
;;;
;;;   (define script (applescript "Sample.scpt"))
;;;   (apply-applescript-proc script "safariFrontURL")
;;;
;;; With `setSafariFrontURL` it is possible to set the URL of the frontmost browser window:
;;;
;;;   (apply-applescript-proc script "setSafariFrontURL" "https://srfi.schemers.org")
;;;   (apply-applescript-proc script "safariFrontURL")
;;;   ==> "https://srfi.schemers.org/"
;;;
;;; Alternatively, it is possible to define procedures representing the sub-routines:
;;;
;;;   (define safari-front-window-bounds (applescript-proc script "safariFrontWindowBounds"))
;;;   (safari-front-window-bounds)
;;;   ==> (97 23 1421 1050)
;;;
;;; Here is an example how to play an Internet radio station via the Music application:
;;;   (play-music-stream "http://protonradio.com:8000/live.m3u")
;;;   (stop-music)
;;;
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
        (lispkit system)
        (lisppad system)
        (lisppad applescript))

;; This is the name of the script that is packaged with LispPad to support this example code.
(define sample-script-file "Sample")

;; Returns the file name for a script with the given name.
(define (script-name name)
  (append-path-extension name "scpt" #t))

;; Install script `sample-script-file` in the systems application scripts folder so that it
;; is accessible by LispPad.
(define (install-sample-script . args)
  (let-optionals args ((overwrite? #f))
    (let* ((name   (script-name sample-script-file))
           (dir    (car (system-directory 'application-scripts)))
           (target (path dir name)))
      (if (or overwrite? (not (file-exists? target)))
          (let ((chosen (show-save-panel "Save script" "Script file:" dir name)))
            (if (string-ci=? target chosen)
                (begin
                  (if (file-exists? target) (delete-file target))
                  (copy-file (asset-file-path sample-script-file "scpt" "Scripts") target))
                (show-message-panel "Wrong directory. Will not install script.")))))))

;; Open a script installed in the application scripts folder with the script editor.
(define (view-script filename)
  (open-file
    (path (car (system-directory 'application-scripts)) (script-name filename))))

;; Install sample script (if needed)

(install-sample-script)
(define script (applescript (script-name sample-script-file)))

;; Define all sub-routines as regular Scheme procedures

; Safari

(define safari-urls                    (applescript-proc script "safariURLs"))
(define safari-front-URL               (applescript-proc script "safariFrontURL"))
(define set-safari-front-URL           (applescript-proc script "setSafariFrontURL"))
(define safari-front-window-bounds     (applescript-proc script "safariFrontWindowBounds"))
(define set-safari-front-window-bounds (applescript-proc script "setSafariFrontWindowBounds"))

; Music

(define count-music-tracks             (applescript-proc script "countMusicTracks"))
(define search-music-library           (applescript-proc script "searchMusicLibrary"))
(define get-music-tracks               (applescript-proc script "getMusicTracks"))
(define all-music-tracks               (applescript-proc script "allMusicTracks"))
(define play-music-track               (applescript-proc script "playMusicTrack"))
(define play-music-track-id            (applescript-proc script "playMusicTrackId"))
(define stop-music                     (applescript-proc script "stopMusic"))
(define toggle-music                   (applescript-proc script "toggleMusic"))
(define play-music-stream              (applescript-proc script "playMusicStream"))
