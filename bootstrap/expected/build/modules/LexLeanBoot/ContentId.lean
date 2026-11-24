module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanStd.Blake3
public import LexLeanStd.Numeral
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanBoot.ContentId

@[expose] public def lit (literal : String) : ByteArray := (LexLeanRuntime.utf8Encode (literal) : ByteArray)

@[expose] public def cat (left : ByteArray) (right : ByteArray) : ByteArray := (LexLeanRuntime.append (left) (right) : ByteArray)

@[expose] public def byteText (value : Nat) : ByteArray := LexLeanStd.Numeral.pick (LexLeanStd.Numeral.byteValues) (value)

@[expose] public def bigEndian (value : Nat) (width : Nat) : ByteArray := ((LexLeanCollections.iterate ((fun (state : (Prod (Nat) (ByteArray))) => ((LexLeanRuntime.quotient ((state).1) (256) (0) : Nat), cat (byteText ((LexLeanRuntime.remainder ((state).1) (256) (0) : Nat))) ((state).2)))) (width) ((value, _root_.ByteArray.mk #[])) : (Prod (Nat) (ByteArray)))).2

@[expose] public def frame (label : ByteArray) (payload : ByteArray) : ByteArray := cat (cat (cat (bigEndian ((LexLeanRuntime.length (label) : Nat)) (4)) (label)) (bigEndian ((LexLeanRuntime.length (payload) : Nat)) (8))) (payload)

@[expose] public def framedHash (domain : ByteArray) (frames : ByteArray) : ByteArray := LexLeanStd.Blake3.blake3Label (cat (cat (domain) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 0))])) (frames))

@[expose] public def treeDigest (files : List ((Prod (ByteArray) (ByteArray)))) : ByteArray := framedHash (lit ("lexlean-tree-v2")) ((LexLeanCollections.listFold ((fun (acc : ByteArray) (row : (Prod (ByteArray) (ByteArray))) => cat (acc) (frame ((row).1) ((row).2)))) (_root_.ByteArray.mk #[]) (files) : ByteArray))

@[expose] public def sourceId (projectToml : ByteArray) (lockText : ByteArray) (sources : List ((Prod (ByteArray) (ByteArray)))) : ByteArray := framedHash (lit ("lexlean-source-v2")) (cat (cat (frame (lit ("project")) (projectToml)) (frame (lit ("lock")) (lockText))) ((LexLeanCollections.listFold ((fun (acc : ByteArray) (row : (Prod (ByteArray) (ByteArray))) => cat (acc) (cat (frame (lit ("path")) ((row).1)) (frame (lit ("source")) ((row).2))))) (_root_.ByteArray.mk #[]) (sources) : ByteArray)))

@[expose] public def semanticId (compilerSemantics : ByteArray) (language : ByteArray) (linkedIr : ByteArray) (lexiconClosure : ByteArray) : ByteArray := framedHash (lit ("lexlean-semantic-v2")) (cat (cat (cat (cat (frame (lit ("compiler-semantics")) (compilerSemantics)) (frame (lit ("language")) (language))) (frame (lit ("toolchain")) (lit ("leanprover/lean4:v4.32.2")))) (frame (lit ("linked-ir")) (linkedIr))) (frame (lit ("lexicon-closure")) (lexiconClosure)))

@[expose] public def buildId (sourceDigest : ByteArray) (semanticDigest : ByteArray) : ByteArray := framedHash (lit ("lexlean-build-v2")) (cat (frame (lit ("source-id")) (sourceDigest)) (frame (lit ("semantic-id")) (semanticDigest)))

@[expose] public def attestationId (attestationBody : ByteArray) : ByteArray := framedHash (lit ("lexlean-attestation-v2")) (frame (lit ("attestation-body")) (attestationBody))

@[expose] public def pdfRecipeId (texDigest : ByteArray) (programDigest : ByteArray) (versionDigest : ByteArray) (compileArgv : ByteArray) (resources : ByteArray) : ByteArray := framedHash (lit ("lexlean-pdf-recipe-v2")) (cat (cat (cat (cat (frame (lit ("tex")) (texDigest)) (frame (lit ("program")) (programDigest))) (frame (lit ("version-output")) (versionDigest))) (frame (lit ("argv")) (compileArgv))) (frame (lit ("resources")) (resources)))

end LexLeanBoot.ContentId
