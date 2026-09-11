## areas
- id: uuid
- name: text

## checkpoints
- id: uuid
- name: text
- number: integer number
- category: enum

- location: text
- area: foreign key, ref areas
- coordinates: lat,long
- accessible: bool
- lanes: number
- checkpoint_description: jsonb, language separated object of text / tiptap json
- org_description: jsonb, language separated
- url: text

- cancelled: bool

**private fields, only for admins**:
- requirements: text
- execution: text

- contact_person: string
- contact_email: email
- contact_phone: phone numb

## teams
- id: uuid
- name: text
- participants: integer

## scores
- id: uuid
- team: foreign key, references a team
- checkpoint: foreign key, references a checkpoint
- score: integer (0-12)
- participants: integer

## news
- id: uuid
- title: text
- content: jsonb, language separated tiptap json
- published_at: date

## photos
- id: uuid
- url: s3-object

## votes
- photo: foreign_key, ref photos

## checkpoint_rating
- id: uuid
- rating: int (0-5)
- checkpoint: fk, ref checkpoint

## team_rating
- id: uuid
- rating: int (0-5)
- team: fk, ref team

## checkpoint_report
- id: uuid
- checkpoint: fk, ref checkpoint
- title: text
- content: jsonb, tiptap json

## team_report
- id: uuid
- team: fk, ref team
- title: text
- content: jsonb, tiptap json







