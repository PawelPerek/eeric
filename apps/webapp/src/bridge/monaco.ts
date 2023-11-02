import { editor, languages, MarkerSeverity, Range } from "monaco-editor";
import { monarchDefinition } from "./monarch";

const RISCV = "risc-v";

languages.setMonarchTokensProvider(RISCV, monarchDefinition);
languages.register({
  id: RISCV,
  extensions: ["S"]
})

let finishLoadMonaco: (e: editor.IStandaloneCodeEditor) => void;
const loadMonaco: Promise<editor.IStandaloneCodeEditor> = new Promise(resolve => {
  finishLoadMonaco = resolve;
});


export function create(parent: HTMLElement) {
  const monaco = editor.create(parent, {
    language: RISCV,
    fontSize: 18,
    theme: "vs-dark"
  });

  finishLoadMonaco(monaco);

  const observer = new ResizeObserver((entries) => {
    for (const entry of entries) {
      const { width, height } = entry.contentRect;
      monaco.layout({ width, height })
    }
  })

  observer.observe(parent);
}

let lastCode = "";

export async function onInput(listener: (value: string) => void) {
  const monaco = await loadMonaco;
  monaco.getModel().onDidChangeContent(_ => {
    const code = monaco.getValue();
    if (code != lastCode) {
      listener(code);
      lastCode = code;
    }
  });
}

export async function disable() {
  const monaco = await loadMonaco;
  monaco.updateOptions({ readOnly: true });
}

export async function enable() {
  const monaco = await loadMonaco;
  monaco.updateOptions({ readOnly: false });
}

let collections: editor.IEditorDecorationsCollection | undefined;

export async function highlightLine(line: number) {
  const monaco = await loadMonaco;
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
    ])
  }
}

export async function setInput(code: string) {
  const monaco = await loadMonaco;
  if (code != lastCode) {
    monaco.setValue(code);
  }
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

export async function setErrors(lines: number[], errorMessages: string[]) {
  const monaco = await loadMonaco;
  const model = monaco.getModel();

  const errorMarkers: editor.IMarkerData[] = zip(lines, errorMessages)
    .map(([lineNumber, errorMessage]) => {
      const trueLineNumber = lineNumber + 1;

      const [startColumn, endColumn] = measureInstructionLength(trueLineNumber, model);

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