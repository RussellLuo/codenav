/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export const enum Language {
  Python = 0,
  JavaScript = 1,
  TypeScript = 2
}
export interface Point {
  line: number
  column: number
}
export interface Span {
  start: Point
  end: Point
}
export const enum TextMode {
  Overview = 0,
  Complete = 1
}
export interface Reference {
  /** File path */
  path: string
  /** Position line (0-based) */
  line: number
  /** Position column (0-based grapheme) */
  column: number
  /** The text string */
  text: string
}
export declare class Definition {
  path: string
  span: Span
  text(mode: TextMode): string
}
export declare class Navigator {
  constructor(language: Language, dbPath: string, verbose: boolean)
  index(sourcePaths: Array<string>, force: boolean): void
  clean(delete: boolean): void
  resolve(reference: Reference): Array<Definition>
}
export declare class Snippet {
  constructor(path: string, lineStart: number, lineEnd: number)
  references(queryPath: string): Array<Reference>
  contains(d: Definition): boolean
}
