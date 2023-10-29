import { editor, languages, MarkerSeverity, Range } from "monaco-editor";
import { monarchDefinition } from "./monarch";

const RISCV = "risc-v";

languages.setMonarchTokensProvider(RISCV, monarchDefinition);
languages.register({
  id: RISCV,
  extensions: ["S"]
})

let monaco: editor.IStandaloneCodeEditor;

export function create(parent: HTMLElement) {
  monaco = editor.create(parent, {
    language: RISCV,
    fontSize: 18,
    theme: "vs-dark"
  });

  let observer = new ResizeObserver((entries) => {
    for (const entry of entries) {
      let { width, height } = entry.contentRect;
      monaco.layout({ width, height })
    }
  })

  observer.observe(parent);
}

export function onInput(listener: (value: string) => void) {
  monaco.getModel().onDidChangeContent(_ => {
    let code = monaco.getValue();
    listener(code)
  });
}

export function disable() {
  monaco.updateOptions({ readOnly: true });
}

export function enable() {
  monaco.updateOptions({ readOnly: false });
}

let collections: editor.IEditorDecorationsCollection | undefined;

export function highlightLine(line: number) {
  collections?.clear();

  if (line != 0) {
    collections = monaco.createDecorationsCollection([
      {
        range: new Range(line, 1, line, 1),
        options: {
          isWholeLine: true,
          className: "highlighted-line",
        },
      },
    ]);
  }
}

export function setInput(code: string) {
  monaco.setValue(code);
}

const zip = <T, U>(a: Array<T>, b: Array<U>): Array<[T, U]> => a.map((value, index) => [value, b[index]]);

function measureInstructionLength(line: number, model: editor.ITextModel): [number, number] {
  const content = model.getLineContent(line);
  const contentWithoutComment = content.split("#")[0];

  const startIndex = contentWithoutComment.search(/\S/);

  let endIndex = contentWithoutComment.length - 1;
  while (endIndex >= 0 && /\s/.test(contentWithoutComment[endIndex])) {
    endIndex--;
  }

  return [startIndex + 1, endIndex + 2];
}

export function setErrors(lines: number[], errorMessages: string[]) {
  let model = monaco.getModel();

  const errorMarkers: editor.IMarkerData[] = zip(lines, errorMessages)
    .map(([lineNumber, errorMessage]) => {
      let trueLineNumber = lineNumber + 1;

      let [startColumn, endColumn] = measureInstructionLength(trueLineNumber, model);

      return {
        message: errorMessage,
        severity: MarkerSeverity.Error,
        startLineNumber: trueLineNumber,
        endLineNumber: trueLineNumber,
        startColumn,
        endColumn
      }
    });

  editor.setModelMarkers(model, "Compiler", errorMarkers)
}