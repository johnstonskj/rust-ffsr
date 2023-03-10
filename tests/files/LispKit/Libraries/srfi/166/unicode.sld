;;; SRFI 166 UNICODE
;;; Monadic Formatting
;;;
;;; A library of procedures for formatting Scheme objects to text in various ways, and
;;; for easily concatenating, composing and extending these formatters efficiently
;;; without resorting to capturing and manipulating intermediate strings. This SRFI is
;;; an updated version of SRFI 159, primarily with the difference that state variables
;;; are hygienic.
;;;
;;; Copyright © 2020 Marc Nieper-Wißkirchen. All rights reserved.
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
;;; INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
;;; PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
;;; HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF
;;; CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE
;;; OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
;;;
;;; LispKit Port:
;;;   Copyright © 2021 Matthias Zenger. All rights reserved.

(define-library (srfi 166 unicode)
  
  (export as-unicode
          unicode-terminal-width
          unicode-terminal-width/wide)
  
  (import (scheme base)
          (srfi 1)
          (srfi 151)
          (srfi 166 base))
        
  (begin
    (define (uc-width code ambiguous-is-wide?)
      (cond
        ((zero-width? code) 0)
        ((full-width? code) 2)
        ((and ambiguous-is-wide? (ambiguous-width? code)) 2)
        (else 1)))
    
    ;; TODO: The following procedures have been more or less copied from
    ;; the Chibi Scheme implementation. They really should go into a
    ;; generally usable Unicode SRFI.
    
    (define (zero-width? n)
      (if (< n 68900)
          (if (< n 12330)
              (if (< n 6679)
                  (if (< n 1425)
                      (if (< n 1155) (<= 768 n 879) (<= n 1159))
                      (if (> n 4253)
                          (if (< n 5906)
                              (<= 4957 n 4959)
                              (if (> n 6459)
                                  #f
                                  (bit-set? (- n 5906) 51654814515775487143066232783390167417190249239197745817200662630546961064292554504203649883705840324554781044118350731330848841429221578063291283522087222079752503303))) (bit-set? (- n 1425) 205510703248794064385799288111379553899177236721844427958663885943912447037636015703722458582080212074964115397630065758419083258557178791291572045390928965000517761101693657384097462852805005223789556257652925846392379688350478539272364567836963922093122354965736916957801582655554045063883899910493538605405067002499688533586876666092658246104323699888852680426382228126850348432897066859954019357227298272892506279761774951117859456904408554199637716442671276335693418641886045354548080235810360809122448516863904652167066224711146510448624096937959409199684953654914585698643427248887519665328616615751486909865868764846766218624841700373624128912795005115413173034267739971653354314214069620902510761561645618122027617911016679702936039677009392664194441428497838216197914975998358752016720492749295864086894236325597101175925631693488402676580351))) (if (> n 7223) (if (< n 8400) (if (< n 7616) (if (< n 7376) #f (if (> n 7417) #f (bit-set? (- n 7376) 3367824654327))) (if (> n 7679) #f (bit-set? (- n 7616) 18158513697557839871))) (if (> n 8432) (if (< n 11647) (<= 11503 n 11505) (if (> n 11775) #f (bit-set? (- n 11647) 680564733683420601898220539676448522241))) (bit-set? (- n 8400) 8587976703))) (bit-set? (- n 6679) 93549288715282589785155353610353886491418033273055579755950049941276640743523826260726496451218508777751751117752658787002603454722245250226862282085717170773295123))) (if (> n 12442) (if (< n 65024) (if (< n 43204) (if (< n 43010) (if (< n 42607) #f (if (> n 42737) #f (bit-set? (- n 42607) 2041694201525630780780248066803074367457))) (if (> n 43052) #f (bit-set? (- n 43010) 4501125726737))) (if (> n 43766) (if (< n 64286) (if (< n 44005) #f (if (> n 44013) #f (bit-set? (- n 44005) 265))) (<= n 64286)) (bit-set? (- n 43204) 15140075821452045156435686692227855958090094076212194752904600649738386873024625209829508281199675047875687625406266463651325109390207641183187724769853784365253770870787))) (if (> n 65071) (if (< n 66422) (if (< n 66272) (<= 66045 n 66045) (<= n 66272)) (if (> n 66426) (if (< n 68325) (if (< n 68097) #f (if (> n 68159) #f (bit-set? (- n 68097) 4863887597560166455))) (<= n 68326)) #t)) (bit-set? (- n 65024) 281470681808895))) (bit-set? (- n 12330) 7788445287802241442795744493830159))) (if (> n 68903) (if (< n 92912) (if (< n 71090) (if (< n 69633) (if (< n 69446) (<= 69291 n 69292) (<= n 69456)) (if (> n 70206) (if (< n 70712) (if (< n 70367) #f (if (> n 70516) #f (bit-set? (- n 70367) 1388177832465565583748880650475847749063413745))) (if (> n 70851) #f (bit-set? (- n 70712) 1178046920456322681183219660472574183234815))) (bit-set? (- n 69633) 31315396710562944755562359524395422889373597990417327292477766884025457959630346593353500376569566606431910948292248640666923116843972687831060900891883862689184984344821761))) (if (> n 71467) (if (< n 72148) (if (< n 71995) (if (< n 71727) #f (if (> n 71738) #f (bit-set? (- n 71727) 3583))) (if (> n 72003) #f (bit-set? (- n 71995) 267))) (if (> n 72345) (if (< n 73459) (if (< n 72752) #f (if (> n 73111) #f (bit-set? (- n 72752) 1495361097625526625570256678500368769738841504996522270098150646281731627800177929801827597371100995086499711))) (<= n 73460)) (bit-set? (- n 72148) 351511567199490659298273789834301951705775336083122752065743))) (bit-set? (- n 71090) 605567007883067350144846792920936979368360383438593457381381338194386513335133428523310644367613386859332205833231))) (if (> n 92982) (if (< n 122880) (if (< n 119143) (if (< n 113821) (if (< n 94031) #f (if (> n 94180) #f (bit-set? (- n 94031) 713623846352979940529143261425908673834647553))) (<= n 113822)) (if (> n 119213) (if (< n 121344) (<= 119362 n 119364) (if (> n 121519) #f (bit-set? (- n 121344) 95779464130560000838435563242757336329136322539159551))) (bit-set? (- n 119143) 2213609288981778792455))) (if (> n 122922) (if (< n 125136) (if (< n 123628) (<= 123184 n 123190) (<= n 123631)) (if (> n 125258) (<= 917760 n 917999) (bit-set? (- n 125136) 10550747216542769741173968540975235199))) (bit-set? (- n 122880) 8641373536127))) (bit-set? (- n 92912) 2342736497361113055263))) #t)))
    
    (define (full-width? n)
      (if (< n 65504) (if (< n 12880) (if (< n 11035) (if (< n 9193) (if (< n 8986) (<= 4352 n 4447) (if (> n 9002) #f (bit-set? (- n 8986) 98307))) (if (> n 9203) (if (< n 9725) #f (if (> n 10175) #f (bit-set? (- n 9725) 2907443622617266820054333480998780529725701130302280275370007947623808338047781548019528101403890814668593761726693391537327403448664067))) (bit-set? (- n 9193) 1167))) (if (> n 11093) (if (< n 12288) (if (< n 11904) #f (if (> n 12284) #f (bit-set? (- n 11904) 2462024160382423828429811516068427561660749306784056468848805869416245321611661291646414375862022918616033934704639))) (if (> n 12288) (if (< n 12289) #f (if (> n 12879) #f (bit-set? (- n 12289) 31658291388542983835601784359821364092920312042945607229551814400819120094685286422379319320933806533062611154529989222389109195010548367461806701052374837219444518916037017599))) #t)) (bit-set? (- n 11035) 297237575406452739))) (if (> n 19903) (if (< n 44032) (if (< n 42128) (if (< n 19968) (if (< n 19904) #f (if (> n 19967) #f (bit-set? (- n 19904) 0))) (<= n 42124)) (if (> n 42182) (<= 43360 n 43388) #t)) (if (> n 55203) (if (< n 65040) (<= 63744 n 64255) (if (> n 65131) (<= 65281 n 65376) (bit-set? (- n 65040) 4797017504656895971262727167))) #t)) #t)) (if (> n 65510) (if (< n 127183) (if (< n 101632) (if (< n 100344) (if (< n 94208) (if (< n 94176) #f (if (> n 94207) #f (bit-set? (- n 94176) 196639))) (<= n 100343)) (if (> n 100351) (<= 100352 n 101589) (bit-set? (- n 100344) 0))) (if (> n 101640) (if (< n 110960) (if (< n 110592) #f (if (> n 110959) #f (bit-set? (- n 110592) 2201759651238793353935978839989206105180151758945528329367307032632076306562609289056077994868130232108318719))) (if (> n 111355) (<= 126980 n 126980) #t)) #t)) (if (> n 127183) (if (< n 129292) (if (< n 127744) (if (< n 127374) #f (if (> n 127589) #f (bit-set? (- n 127374) 103666862632257055884375554338292758824945695871569254787261145081))) (if (> n 128764) (<= 128992 n 129003) (bit-set? (- n 127744) 22427532355441897097494388929338899236988773969297598703485634661859864729808161736681853448048691489805457651039471569590476931512946609416855878944269320917585568431313760949188816496022192318927601521301355797906472807547744415626581438889327443126299259883097149821600460467129283287639251279166002167807))) (if (> n 129750) (if (< n 196606) (<= 131072 n 196605) (if (> n 196607) (<= 196608 n 262141) (bit-set? (- n 196606) 0))) (bit-set? (- n 129292) 1476937530268593348028592145760648876257390306909878067362834750498663701692341803996798291958154966950849039520561259075909404413382361087))) #t)) #t)))
    
    (define (ambiguous-width? n)
      (if (< n 65024) (if (< n 11094) (if (< n 8208) (if (< n 161) #f (if (> n 1105) #f (bit-set? (- n 161) 223052536271667459415678930318905079873549773617932793730641638019059504712673796525524909775020557665956016104500455950219001349342694128220978917003401578883591589344111031259581620889226429090957790179652065257339299797525743429872558723447631104797406273409853160160039741152015049))) (if (> n 8978) (if (< n 9312) #f (if (> n 10111) #f (bit-set? (- n 9312) 6661502700035245041435504908796213048795246635156488431799991666791867984381040418533146610100595617817495711540720281547788461940893726414925833870538641183879792268453003453351317627145210838405747057167482906429027337302253989020846522367))) (bit-set? (- n 8208) 6210072369202835740595918595956453086120179989637780933534686381158779319390650106752194038767228255040840848246999248665204356172037404661019798426289847167202141507582321755409018753079304131847016847989633301370435793517521875833))) (if (> n 11097) (if (< n 57344) (<= 12872 n 12879) (<= n 63743)) #t)) (if (> n 65039) (if (< n 983040) (if (< n 127232) (<= 65533 n 65533) (if (> n 127404) (<= 917760 n 917999) (bit-set? (- n 127232) 11972575780114894207525815562028054143265854222960639))) (if (> n 1048573) (if (< n 1048576) (if (< n 1048574) #f (if (> n 1048575) #f (bit-set? (- n 1048574) 0))) (<= n 1114109)) #t)) #t)))
    )
  
  (begin ; "unicode.scm"
    (define (as-unicode . fmt*)
      (fn (ambiguous-is-wide?)
        (with ((string-width (if ambiguous-is-wide?
                     unicode-terminal-width/wide
                     unicode-terminal-width)))
          (each-in-list fmt*))))
    
    ;;; FIXME: Implement ANSI escape sequences.
    (define (%unicode-terminal-width str wide?)
      (let loop ((chars (string->list str)) (width 0))
        (cond ((null? chars) width)
          ((and (not (null? (cdr chars)))
            (char=? (car chars) #\x1b)
            (char=? (cadr chars) #\[))
           (let loop/escape ((chars (cddr chars)))
             (cond ((null? chars) width)
               ((char=? (car chars) #\m) (loop (cdr chars) width))
               (else (loop/escape (cdr chars))))))
          (else
           (loop (cdr chars) (+ width
                    (max 0 (uc-width (char->integer (car chars))
                             wide?))))))))
    
    (define (unicode-terminal-width str)
      (%unicode-terminal-width str #f))
    
    (define (unicode-terminal-width/wide str)
      (%unicode-terminal-width str #t))
  )
)
