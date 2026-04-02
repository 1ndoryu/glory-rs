"""Regenerate .sqlx cache for the scheduler query after adding ? override to email_cliente."""
import json
import hashlib
import os

OLD_FILE = '.sqlx/query-a8062f8a829190871e768ba44c6f7ae1f6b8310224b8aba00cddc8cfc4857cc2.json'

with open(OLD_FILE, 'r') as f:
    data = json.load(f)

# Update query text: email_cliente -> "email_cliente?"
old_fragment = 'c.email AS email_cliente'
new_fragment = 'c.email AS "email_cliente?"'
data['query'] = data['query'].replace(old_fragment, new_fragment)

# Compute new SHA256 hash
new_hash = hashlib.sha256(data['query'].encode('utf-8')).hexdigest()
data['hash'] = new_hash

# Update column name
for col in data['describe']['columns']:
    if col['name'] == 'email_cliente':
        col['name'] = 'email_cliente?'

# Set nullable for col 13 to true
data['describe']['nullable'][13] = True

# Write new file
new_path = f'.sqlx/query-{new_hash}.json'
with open(new_path, 'w', newline='\n') as f:
    json.dump(data, f, indent=2)
    f.write('\n')

print(f'New file: {new_path}')
print(f'Old file can be deleted: {OLD_FILE}')

# Delete old file
os.remove(OLD_FILE)
print('Old file deleted.')
