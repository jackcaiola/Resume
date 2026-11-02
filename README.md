# Resume

My resume. Content lives in `resume_data.yaml`; the Yew components render it at `/resume/`.

Hover or focus a skill, experience title, or project bullet to show related skills and experience. Click a skill to pin its selection; click it again to clear. Role-level tags belong on the role title; project-specific tags stay on the corresponding bullet.

`Duty.project` and `Duty.project_detail` render nested project headings. `DateRange.display_label` supports dates such as flight training with an unconfirmed completion date. Experience locations may be blank when unspecified.

The PDF download is `JackCaiola-Resume.pdf`, copied into `dist/` by Trunk. Regenerate it from the built site's print view when content changes. A repository commit does not by itself deploy the website.


### Local development

The app is compiled using `trunk`. Local development can be via running:

Opening a nix shell (pulls in required dependencies):
```
nix develop
```

Using trunk to compile and interactively serve the webapp:
```
trunk serve
```

Trunk serve will recompile the webapp if any files are modified.

### Deploying
Run `nix develop .#deploy --command deploy` to enter the browser-enabled deployment environment and publish. The default `nix develop` shell is browser-free so tests do not depend on Chrome downloads. Deployment will:
1. Compile the rust code to a wasm executable
2. Create wasm js wrapper
3. Copy `index.html`, `style.css`, etc into the directory that will become the webserver
4. Run `generate_pdf`, which runs chrome headlessly on a temporaly server
5. Copies these files to `tneubauer.xyz`

