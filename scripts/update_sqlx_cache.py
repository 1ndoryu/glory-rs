import hashlib
import json
import os

os.chdir(r"c:\Users\Owner\OneDrive\Documentos\glory-rust-template")

# The INSERT INTO campanas query — changed from 9 to 10 params
old_insert_file = ".sqlx/query-7ed92d80c16ccf02e66aa06e7c57f51a7b55c6c76dc7983add818517d40b6fe7.json"
# The UPDATE campanas query — added plantilla_whatsapp_id COALESCE
old_update_file = ".sqlx/query-bc13595e14a88aacfb0b79afb53cf5e36d81e3db58bc80b4cabcfbb2c062825f.json"

# New queries — must match exactly what Rust string continuation produces
new_insert_query = (
    "INSERT INTO campanas (id, user_id, nombre, descripcion_interna, cuerpo_mensaje, "
    "canales, segmento, incluir_baja, telefono_baja, plantilla_whatsapp_id) "
    "VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) "
    "RETURNING *"
)

new_update_query = (
    "UPDATE campanas SET "
    "nombre = COALESCE($3, nombre), "
    "descripcion_interna = COALESCE($4, descripcion_interna), "
    "cuerpo_mensaje = COALESCE($5, cuerpo_mensaje), "
    "canales = COALESCE($6, canales), "
    "segmento = COALESCE($7, segmento), "
    "incluir_baja = COALESCE($8, incluir_baja), "
    "telefono_baja = COALESCE($9, telefono_baja), "
    "plantilla_whatsapp_id = COALESCE($10, plantilla_whatsapp_id), "
    "updated_at = NOW() "
    "WHERE id = $1 AND user_id = $2 AND estado = 'borrador' "
    "RETURNING *"
)

def update_sqlx_file(old_path, new_query):
    with open(old_path) as f:
        data = json.load(f)
    
    print(f"Old query: {data['query'][:80]}...")
    print(f"Old hash:  {data['hash']}")
    print(f"Old params: {len(data['describe']['parameters']['Left'])}")
    
    new_hash = hashlib.sha256(new_query.encode()).hexdigest()
    data['query'] = new_query
    data['hash'] = new_hash
    # Add $10 parameter type (Uuid)
    data['describe']['parameters']['Left'].append("Uuid")
    
    new_path = f".sqlx/query-{new_hash}.json"
    with open(new_path, 'w') as f:
        json.dump(data, f, indent=2, ensure_ascii=False)
    os.remove(old_path)
    
    print(f"New hash:  {new_hash}")
    print(f"New params: {len(data['describe']['parameters']['Left'])}")
    print(f"Renamed: {os.path.basename(old_path)} -> {os.path.basename(new_path)}")
    print()

print("=== INSERT ===")
update_sqlx_file(old_insert_file, new_insert_query)

print("=== UPDATE ===")
update_sqlx_file(old_update_file, new_update_query)

print("Done!")
