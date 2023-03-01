export type MessagesMap = {
  readonly [key: string]: PlaceholderMap &
    AttributeMap & {readonly hasValue?: true};
};

export type AttributeMap = {
  readonly attributes: {
    readonly [key: string]: PlaceholderMap;
  };
};

export type PlaceholderMap = {
  readonly placeholders: {
    readonly [key: string]: FluentPlaceholder;
  };
};

export type Attrs<Name extends keyof Messages> = Messages[Name]['attributes'];

export type MessageNames<S extends string = '.'> =
  | keyof {
      readonly [Key in keyof Messages as Messages[Key]['hasValue'] extends true
        ? Key
        : never]: string;
    }
  | keyof {
      readonly [Key in keyof Messages as `${Key}${S}${Extract<
        keyof Attrs<Key>,
        string
      >}`]: Attrs<Key>;
    };

export type Placeholders<
  Name extends MessageNames<Sep>,
  Sep extends string = '.'
> = Name extends keyof Messages
  ? Messages[Name]['placeholders']
  : Name extends `${infer N}.${infer A}`
  ? N extends keyof Messages
    ? A extends keyof Attrs<N>
      ? Attrs<N>[A] extends PlaceholderMap
        ? Attrs<N>[A]['placeholders']
        : never
      : never
    : never
  : never;
