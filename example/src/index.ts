import {FluentBundle, FluentResource} from '@fluent/bundle';
import {readFile} from 'fs/promises';
import path from 'path';

import {Messages} from './messages';

type Locale = 'en-US';

class TypedLocalizer {
  #bundles = new Map<Locale, FluentBundle>();

  async loadFile(locale: Locale, file: string) {
    const src = await readFile(file, 'utf8');
    const resource = new FluentResource(src);

    let bundle = this.#bundles.get(locale);
    if (!bundle) {
      bundle = new FluentBundle(locale, {useIsolating: false});
    }

    bundle.addResource(resource);

    this.#bundles.set(locale, bundle);
  }

  get<T extends keyof Messages>(
    locale: Locale,
    msg: T,
    args?: Messages[T]
  ): string {
    const bundle = this.#bundles.get(locale);
    if (!bundle) {
      return msg;
    }

    const pat = bundle.getMessage(msg);
    if (!pat || !pat.value) {
      return msg;
    }

    return bundle.formatPattern(pat.value, args);
  }
}

const main = async () => {
  const l = new TypedLocalizer();
  await l.loadFile('en-US', path.join(process.cwd(), '..', 'lang.ftl'));

  const msg = l.get('en-US', 'shared-photos', {
    photoCount: 5,
    userGender: 'other',
    userName: 'Ben',
  });

  console.log(msg);
};

main().catch(console.error);
