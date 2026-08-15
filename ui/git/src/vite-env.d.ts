/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_API_BASE_URL: string;
  readonly VITE_GIT_MODULE_URL: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
  readonly vitest?: ImportMetaVitest;
}

interface ImportMetaVitest {
  describe: typeof describe;
  it: typeof it;
  expect: typeof expect;
  vi: typeof vi;
  beforeAll: typeof beforeAll;
  afterAll: typeof afterAll;
  beforeEach: typeof beforeEach;
  afterEach: typeof afterEach;
}
