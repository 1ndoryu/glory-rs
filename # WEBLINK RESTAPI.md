# WEBLINK RESTAPI

##### MANUAL DE USO DE WEBLINK RESTAPI


## Cambios de versión

**30.**^
CreateOrder Se han añadido los campos _Region_ y _Country_ al objeto _Customer_ de _Order_.
GetOrder Se han añadido los campos _Region_ y _Country_ al objeto _Customer_ de _Order_.
**30.**^
GetOrder Posibilidad de obtener una comanda de mesa a partir de los números de salón y de mesa
    mediante los parámetros _RoomNumber_ y _TableNumber_.
CancelOrder Posibilidad de cancelar una comanda a partir de su _MarketplaceOrderId_ y _MarketId_ o
    bien, si es una mesa, a partir de los números de salón y de mesa ( _RoomNumber_ y
    _TableNumber_ ).
GetRoomTables Nueva función para obtener los números de las mesas del rango de un salón.
**30.**^
CreateOrder Se ha añadido el campo _Tip_ (propina) al objeto _Order_.
    Se han añadido los parámetros _Invoice_ y _InvoiceParameters_ para facturar la comanda.
    Se ha añadido el campo _Comments_ a los _Items_ de _XOrder_MenuDataType_ para
    especificar una lista de comentarios a un plato de un menú.
GetOrder Se ha añadido el campo _Tip_ (propina) al objeto _Order_.
    Los parámetros _OrderId_ , _MarketplaceId_ , _MarketId_ , _RoomNumber_ y _TableNumber_ se han
    añadido al nuevo objeto _InvoiceParameters_.
AddOrderTip Nueva función para añadir la propina a una comanda.
AddOrderPayment Nueva función para añadir un pago a una comanda y, opcionalmente, emitir la facturar
    de la misma.
InvoiceOrder Nueva función para facturar una comanda.
CancelOrder Los parámetros _OrderId_ , _MarketplaceId_ , _MarketId_ , _RoomNumber_ y _TableNumber_ se han
    añadido al nuevo objeto _InvoiceParameters_.
**31.**^
ExportManagmentDocumentsByExportProfile Se ha agregado un nuevo comando que permite exportar albaranes y facturas de gestión
    usando un perfil de exportación.
GetRoomsTables Nueva función para obtener los números de las mesas de los rangos de los salones.
CreateDocument Se ha eliminado esta función.
**31.**^
Order Campos de sólo lectura _DiscountAmount_ y _DiscountPercentage_ para devolver el
    descuento de una comanda.
OrderItem Campos de sólo lectura _DiscountAmount_ y _DiscountPercentage_ para devolver el
    descuento de una línea de comandas.
**31.**^
GetOrder Nuevos campos _GroupName_ en _XOrder_MenuItemDataType_ con el nombre del grupo de
    platos del menú y _XOrder_PackItemDataType_ con del grupo del pack.
**31.**^
GetPOS Nueva función para obtener/saber si existe un terminal.
GetPOSes Nueva función para obtener todos los números de terminal configurados.
GetEmployee Nueva función para obtener un empleado.
GetEmployees Nueva función para obtener todos/varios empleados.
GetPOSEmployees Nueva función para obtener la lista de empleados de un terminal.
GetOrder Se han añadido los campos _OpeningEmployee_ y _LastEmployee_ al objeto _Order_ y el campo
    _Employee_ al _OrderItem_.
    Se ha añadido el campo _TenderName_ al objeto _Payment_.

```
32.
CallWaiter Función que, al ser llamada, saca una ventana emergente en el TPV indicando que se
reclama a un camarero en una mesa determinada.
Order Se ha añadido el campo de sólo lectura CreationDate.
OrderItem Se ha añadido el campo de sólo lectura TaxName.
```

```
InvoiceParameters Nuevo parámetro PrintTicket del objeto InvoiceParameters de las funciones
InvoiceOrder , AddOrderPayment y CreateOrder para imprimir la factura.
```
**32.**^
AddOrderTip Nuevo parámetro _AddTip_ para sumar la propina en vez de sustituirla.
GetArticle Se han añadido los campos BuyTAVCode y BuyTAVPer.
ExportArticles Se han añadido los campos Is_Inventoriable, BuyTAVCode y BuyTAVPer.
GetPOSArticlesList Se han añadido los campos BuyTAVCode y BuyTAVPer.
GetFullArticlesList Se han añadido los campos BuyTAVCode y BuyTAVPer.
CreateArticlesAndUpdateProfiles Se han añadido los campos BuyTAVCode y BuyTAVPer.
ModifyArticleAndUpdateProfile Se han añadido los campos BuyTAVCode y BuyTAVPer.
**32.**^
Order Campos _Surcharge, SurchargePct, SurchargeAmount_ y _SurchargePercentage_ para
    establecer y devolver el recargo de una comanda.
OrderItem Campo _Invitation_ para indicar/establecer si una línea de comanda es invitación.
**32.**^
Order Campo _VATIncluded_ que indica si los importes son con impuesto (IVA) incluído.
**32.**^
OrderItem Campo _ProportionNumber_ que indica el número de proporción del artículo.
**32.**^
GetItemsCostPrices
ExportPurchaseNotes

```
Nuevo comando
Nuevo comando
```
**32.**^
ExportDocumentsByExportProfile Eliminado campo IncludeBalances
**32.**^
CreateOrder Se ha agregado el campo BarCode como variable de retorno. Este identificador puede ser
    impreso en un código de barras o un código QR. Al ser leído desde el TPV, este abrirá la
    comanda asociada. Por el momento, únicamente se genera para comandas de tipo mesa.

```
GetOrder Se ha agregado el campo BarCode dentro del identificador de la comanda. Con este
podremos identificar una mesa usando un código de barras generado por nosotros en el
comando CreateOrder.
```
**33.**^
GetFullArticlesList Ampliación de campos, ahora se pueden obtener la información de las proporciones de
    un artículo y de sus combinados.
GetPOSSeriesList Nuevo comando que permite obtener la lista de series que dispone el TPV, así como a los
    tipos de documento a los que pertenecen.

```
GetArticle Ampliación de campos sobre combinados y proporciones de un artículo.
```
```
GetPOSArticlesList Ampliación de campos sobre combinados y proporciones de un artículo.
```
```
CreateArticlesAndUpdateProfiles Ampliación de campos sobre combinados y proporciones de un artículo.
```
```
ModifyPricesArticles Ampliación de campos de precios sobre combinados y proporciones de un artículo.
Añadidos también los precios y descuentos de rebajas en Talla y Color.
```
ModifyArticleAndUpdateProfile^ Ampliación^ de^ campos^ sobre^ combinados^ y^ proporciones^ de^ un^ artículo^


```
34.
```
```
GetArticle
ExportArticles
GetPOSArticlesList
GetFullArticlesList
CreateArticlesAndUpdateProfiles
ModifyArticleAndUpdateProfile
```
```
Se ha agregado el campo NotAllowedAsInvitation
Se ha agregado el campo NotAllowedAsInvitation
Se ha agregado el campo NotAllowedAsInvitation
Se ha agregado el campo NotAllowedAsInvitation
Se ha agregado el campo NotAllowedAsInvitation
Se ha agregado el campo NotAllowedAsInvitation
```
**35.**^
Order Campo _VATIncluded_ que indica si los importes son con impuesto (IVA) incluído.
Order
Login

```
Campo AlreadyInvoiced para indicar, en la función CreateOrder , si el pedido ya se factura
en la plataforma en la que se ha creado.
Campo CodigoIntegrador agregado al proceso de Login.
```
## ¿Qué es WebLinkRestApi?

Weblink Rest API es un módulo adicional de BDP-NET que se activa mediante **subscripción extendida**. Aporta todas las
funcionalidades que tiene Weblink además de una serie de funcionalidades que amplían el catálogo de acciones que se pueden
realizar contra BDP-NET.

A diferencia del Weblink, éste está pensado para poder usarse desde servicios que estén alojados en la nube.

## ¿Qué es Weblink Rest API Gratuito?

##### Weblink Rest API Gratuito incluye todos los comandos que incluye Weblink Rest API a excepción de la siguiente lista.

##### Comandos quitados :

- Agregar propina a una comanda existente
- Agregar pago a una comanda existente
- Facturar una comanda existente _(se puede hacer con el CreateOrder marcando la propiedad Invoice = true siempre y_
    _cuando cumpla las condiciones previamente nombradas)_
- Obtener la lista de los salones
- Obtener las mesas de un salón (rangos de mesas)
- Llamar al camarero

##### También se han limitado los siguientes comandos:

##### Obtener Comanda : Limitado

##### El comando únicamente retorna información sobre el estado de la comanda e incluye el siguiente mensaje de error.

[301010]-WEBLINK RESTAPI GRATUITO ÚNICAMENTE RETORNA INFOMACIÓN SOBRE EL ESTADO DE LA COMANDA. PARA OBTENER EL CONTENIDO DE LA
COMANDA PASAR A WEBLINK RESTAPI DE PAGO.

##### Crear Comanda : Limitado

##### El comando no permite la actualización de valores de una comanda existente. Si se está intentando actualizar una

##### comanda retorna el siguiente mensaje de error.

[300041]-WEBLINK RESTAPI GRATUITO NO PERMITE LA ACTUALIZACIÓN DE DATOS DE LA COMANDA, PASE A LA VERSIÓN DE PAGO PARA PODER USAR DICHA
FUNCIONALIDAD



## Preparación del entorno de desarrollo

##### Para la preparación del entorno de desarrollo vamos a precisar de una instalación BDP-NET en su última versión , una

##### subscripción para la activación del aplicativo y una subscripción extendida para la activación del Weblink RestAPI

##### (tanto la versión de pago como la gratuita se activan mediante el uso de subscripciones extendidas).

##### Dichas subscripciones serán ofrecidas por BDP de forma gratuita mientras se realiza el desarrollo o se realiza alguna

##### tarea de integración / depuración del producto (No se permite la comercialización de estas subscripciones).

##### Para su adquisición póngase en contacto con nosotros al teléfono que figura en nuestra página web (972 88 47 65).

##### Este paso requiere información sobre la empresa y su responsable , por lo que tiene que ser él quien se ponga en

##### contacto con nosotros por primera vez. En este paso, también se le facilitará al responsable de la empresa un usuario

##### y una contraseña con el que podrá acceder a la zona de gestión de subscripciones y con los que podrá usar / liberar

##### una subscripción en la aplicación BDP-NET.

### Activación de las subscripciones en BDP-NET

##### Activación BDP-NET

##### Primeramente, tendremos que activar la subscripción de nuestro programa BDP-NET. Para ello, vamos al apartado

##### “Utilidades - > Control de licencias - > Usar Subscripción” y rellenamos los campos solicitados.

- **Nº Subscripción** : 6 primeros dígitos de la subscripción
    (en el caso de un registro 012345-XX-X-XXXXX sería
    012345)
- **Cliente** : Campo descriptivo con el que identificaremos
    donde tenemos activa la subscripción (p.e. Máquina
    Desarrollo)
- **ID. Distribuidor** : Usuario facilitado al responsable de la
    empresa en el momento del alta
    (usuariodummy@bdpuser.com, solo pondremos
    usuariodummy)
- **Password** : Contraseña establecida por el responsable de
    la empresa en el momento del alta.

##### Al pulsar el botón “ USAR SUBSCRIPCIÓN ”, activaremos la licencia en el aplicativo BDP-NET. Podremos saber que todo

##### ha ido correctamente porque nos saldrá un mensaje verde informando que el proceso ha finalizado correctamente y

##### además, notaremos que en la pantalla principal ha desaparecido el cartel de “VERSIÓN DEMO”.


##### Activación subscripción extendida para WeblinkRestAPI

##### Ahora que ya disponemos de una licencia de BDP-NET, podremos activar la subscripción extendida para poder

##### configurar el WeblinkRestAPI. Para ello, nos dirigimos al apartado “ Utilidades - > Control de licencias -> Subscripciones

##### Extendidas ”

##### En la parte superior de la pantalla, disponemos del formulario para poder activar dicha subscripción

- **ID. Distribuidor** : Usuario facilitado al responsable de la empresa en el momento del alta (usuariodummy@bdpuser.com, solo
    pondremos usuariodummy).
- **Password** : Contraseña establecida por el responsable de la empresa en el momento del alta.
- **Nº Subscripción** : 6 primeros dígitos de la subscripción (en el caso de un registro 012345 - XX-X-XXXXX sería 012345)
- **Código Servicio** : 2 siguientes dígitos de la subscripción (en el caso de un registro XXXXXX- 84 - XX-X-XXXXX sería 84)
- **Cliente** : Campo descriptivo con el que identificaremos donde tenemos activa la subscripción (p.e. Máquina Desarrollo)

## Requisitos de funcionamiento y consideraciones adicionales.

WebLinkRestApi requiere un sistema operativo a partir de Windows 7 Service Pack 1. El programa BDP-NET debe funcionar
continuamente en la máquina que realice la función de servidor. Cuando esta máquina se reinicie debido a las actualizaciones u
otras incidencias, el servicio WebLinkRestApi se interrumpirá, lo que debe tenerse en cuenta si se enlaza con un sistema web de
comercio electrónico. Como el servicio WebLinkRestApi debe compartir recursos con el uso normal de los puntos de venta asociados,
el uso intensivo de uno de ellos afectará al rendimiento del otro (y viceversa) por lo que se recomienda instalarlo en una máquina
potente con un acceso de red rápido (por ejemplo, no debe usarse en un punto de venta de gama baja o con recursos limitados).
Debido a lo comentado en el párrafo anterior, WebLinkRestApi no sustituye a un ERP en las webs de comercio electrónico con un
volumen de transacciones moderado o alto. Los enlaces de comercio electrónico disponibles en los ERPs son servicios altamente
especializados para un alto volumen de transacciones que suelen ejecutarse en un hardware específico de alta disponibilidad y alto
rendimiento.

## Configuración del servidor

El primer paso para configurar el sistema WebLinkRestApi es configurar el servidor. Dicho servidor está incorporado dentro de la
propia aplicación BDP-NET. Par acceder al mantenimiento de configuración del servicio WebLinkRestApi se debe accede desde el
menú principal de la aplicación Bdp-Net, a través del menú Utilidades -> Configuración Servicios Web. Para más información acerca
de cómo configurar el servidor WebLinkRestApi se puede consultar la ayuda disponible en la aplicación Bdp-Net para ese apartado.

## De qué manera los clientes deben acceder a los comandos

Los clientes que quieran acceder al servicio WebLinkRestAPI deberá realizar consultas a través de URLs. La dirección a la que los
clientes deberán acceder para obtener datos del servidor Bdp-Net que ejecuta el servicio WebLinkRestAPI corresponderá a la
dirección pública de la conexión a internet en la que se encuentra el servidor Bdp-Net.


El servidor Bdp-Net que ejecuta el servicio WebLinkRestAPI recibirá peticiones a través de un puerto (configurable en la
configuración del apartado de configuración). A la URL de petición de datos a la que tendrán que acceder los clientes de dich o
servicio se le deberá añadir dicho puerto. Será necesario asegurarse que en la ubicación en la que se encuentra el servidor Bdp-Net,
el puerto configurado sea accesible desde el exterior (puerto abierto en el router y en el posible firewall del sistema operativo que
ejecute la aplicación Bdp-Net que actúa de servidor WebLinkRestAPI).

Cada comando al que podrán acceder los clientes corresponderá a una URL diferente. Lo que variará el dicha URL será la parte final.
La parte que variará será lo que se llama la ruta.

Todas las consultas que realicen los clientes a través de estas URLs deberán ser efectuada con el tipo de consulta POST, y, tanto el
formato de datos de entrada de solicitud como las respuestas se efectuará en formato JSON.

##### Nota

En caso de que en el servidor se hubiera configurado que el servicio debe ejecutarse de manera segura (con login y password), el
cliente, en primer lugar, deberá realizar una llamada al comando Login suministrando los datos de autentificación para establecer
una sesión. Es importante puntualizar que trabajar sin Login obligatorio sólo debería utilizarse en un **escenario de desarrollo** y **nunca
en un escenario de funcionamiento real**.


## Comandos y modelos disponibles

### Categoría Servicios

1. ServiceHealth
2. GetVersion
3. GetApplicationVersion
4. Login

### Categoría Artículos

1. GetArticle
2. GetPricesArticles
3. ExportArticles
4. GetPOSArticlesList
5. GetFullArticlesList
6. CreateArticlesAndUpdateProfiles
7. ModifyPricesArticles
8. ModifyArticleAndUpdateProfile

### Categoría Clientes

1. ExportCustomers
2. CreateCustomer

### Categoría Comandas

1. CreateOrder
2. GetOrder
3. CancelOrder
4. AddOrderTip
5. AddOrderPayment
6. InvoiceOrder

### Categoría Comentarios

1. GetCommetsProfile

### Categoría Departamentos

1. ExportDepartment
2. DepartmentsExportFromProfile
3. CreateDepartment
4. CreateDepartmentAndupdateProfiles


### Categoría Menús

1. GetMenuDefinition

### Categoría Fast-Foods

1. GetFastfoodDefinition

### Categoría Packs

1. GetPackDefinition

### Categoría Fidelización y Puntos

1. GetPoints
2. AddPoints

### Categoría Terminales

1. GetPOS
2. GetPOSes

### Categoría Empleados

1. GetEmployee
2. GetEmployees
3. GetPOSEmployees

### Categoría Formas de Pago

1. GetTenderList
2. GetPOSTenderList

### Categoría Perfiles de Departamentos y Artículos

1. GetProfilesListCreateDepartmentList
2. GetProfilesListCreateArticleList
3. GetProfileListModifyArticleList

### Categoría Perfiles de Exportación

1. ExportDocumentsByExportProfile
2. ExportStockAndSalesSummaryByExportProfile
3. ExportManagmentDocumentsByExportProfile
4. ExportPurchaseNotes


### Categoría Stock

1. CreateFamily
2. CreateSubfamily
3. GetStock
4. GetListStock
5. GetItemCostPrices
6. GetItemsCostPrices
7. Regularizations
8. Transfers
9. UpdateMassiveStock
10. UpdateStock
11. UpdateMassiveInventory

### Categoría Suplementos

1. GetSupplementsProfiles
2. GetPOSSupplementsProfile

### Categoría Talla y Color

1. GetInfoSAC
2. GetItemSAC

### Categoría Salones

1. GetRoomTables
2. GetRoomsTables

### Categoría Series TPV

1. GetPOSSeriesList

### Modelos

1. Order
2. OrderCustomer
3. OrderItem
4. OrderItemComment
5. OrderItemSupplement
6. OrderItemTypeMetaInfo
7. Employee
8. OrderPayment
9. InvoiceParameters
10. InvoiceBillingDetails
11. OrderIdentifier
12. POS


13. Room
14. Tender


## Categoría Servicios

En esta categoría se incluirán todos los comandos que permitirán interactuar con el propio servicio WebLinkRestApi que se ejecuta
en la aplicación BDP-Net así como obtener información del funcionamiento de la aplicación. Los comandos disponibles para esta
categoría serán los siguientes:

1. **ServiceHealth** : Comprueba el estado del servicio WeblinkRestAPI.
2. **GetVersion** : Devuelve la versión de la aplicación BDP-Net.
3. **GetApplicationVersion** : Devuelve el estado y la versión de la suscripción extendida de la aplicación BDP-Net
4. **Login** : Realiza el login de un usuario para acceder al servicio de WeblinkRestAPI.


### ServiceHealth

Comando que informará si el servicio WeblinkRestAPI se encuentra funcionando correctamente en la aplicación BDP-Net que actúa
de servidor.

##### Ruta de llamada

/Service/Health

##### Parámetros de entrada

Este comando no posee parámetros de entrada.

##### JSON de solicitud

{
}

##### Parámetros de salida

**IsAlive** **_(Bool)_** : Valor de tipo lógico que indicará si el servicio WebLinkRestAPI está funcionando corretamente (true) en el servidor
BDP-Net o por el contrario no está funcionando correctamente.

##### JSON de respuesta

{
"IsAlive": true
}


### GetVersion

Comando que informará de la versión de BDP-Net que está ejecutando el servicio WebLinkRestAPI.

##### Ruta de llamada

/Service/GetVersion

##### Parámetros de entrada

Este comando no posee parámetros de entrada.

##### JSON de solicitud

{
}

##### Parámetros de salida

**Version** **_(integer)_** : Valor numérico que indicará la versión de la aplicación Bdp-Net que ejecuta el servicio WebiLinkRestAPI.

**SubVersion(integer):** Valor numérico que indicará la subversión de la aplicación Bdp-Net que ejecuta el servicio WebLinkRestAPI.

**Revision(string):** Cadena de caracteres que muestra la posible revisión de la aplicación Bdp-Net que ejecuta el servicio
WebLinkRestAPI.

**Application(string):** Valor numérico que especificará el código de aplicación de la aplicación Bdp-Net que ejecuta el servicio
WebLinkRestAPI.

**ApplicationDescription(string):** Cadena de caracteres que mostrará la descripción del tipo de aplicación correspondiente a la
aplicación Bdp-Net que ejecuta el servicio WebLinkRestAPI.

**ErrorMessage** **_(string)_** : Cadena que almacenará posibles errores. Si la consulta se realizó correctamente, estará vacía.

##### JSON de respuesta

{
"Version": 30,
"Subversion": 1,
"Revision": "b",
"Application": "08",
"ApplicationDescription": "Talla y Color",
"ErrorMessage": ""
}


### GetApplicationVersion

Comando que informará del estado y versión de suscripción extendida solicitada de la que dispone la aplicación Bdp - Net que está
ejecutando el servicio WebLinkRestAPI.

##### Ruta de llamada

/Service/GetApplicationVersion

##### Parámetros de entrada

**Application** **_(integer)_** : Valor numérico con el que se podrá especificar el código de suscripción extendida para la que se desea obtener
la información.

##### JSON de solicitud

{
"Application": 84
}

##### Parámetros de salida

**Version** **_(integer)_** : Valor numérico que indicará la versión de la suscripción extendida solicitada que está asociada a la aplicacion Bdp-
Net que ejecuta el servicio WebiLinkRestAPI.

**SubVersion(integer):** Valor numérico que indicará la subversión de la suscripción extendida solicitada que está asociada a la
aplicación Bdp-Net que ejecuta el servicio WebLinkRestAPI.

**Revision(string):** Cadena de caracteres que muestra la posible revisión de la suscripción extendida solicitada que está asociada a la
aplicación Bdp-Net que ejecuta el servicio WebLinkRestAPI.

**Application(string):** Valor numérico que mostrará el código de suscripción extendida para la que se ha solicitado la información.

**ApplicationDescription(string):** Cadena de caracteres que mostrará la descripción de la suscripción extendida solicitada que está
asociada a la aplicación Bdp-Net que ejecuta el servicio WebLinkRestAPI.

**ErrorMessage** **_(string)_** : Cadena que almacenará posibles errores. Si la consulta se realizó correctamente, estará vacía.

##### JSON de respuesta

{
"Version": 1,
"Subversion": 1,
"Revision": "",
"Application": "84",
"ApplicationDescription": "WeblinkRestAPI",
"ErrorMessage": ""
}


### Login

La aplicación ofrece la posibilidad de que la comunicación entre el cliente del servicio WebLinkRestAPI y el Bdp-Net que ejecuta
dicho servicio, se realice de manera segura y encriptada. En caso de que el servidor WebLinkRestAPI se haya configurado con
seguridad, mediante este comando, el cliente que use dicho servicio podrá ejecutar la acción de Login para poder tener acceso al
servicio.

##### Ruta de llamada

/Auth/Login

##### Parámetros de entrada

**Login** **_(string)_** : Cadena de caracteres en la que se tendrá que especificar el mismo nombre de usuario que se configuró en el servidor
WebLinkRestAPI para acceder a este servicio.

**Password** **_(string)_** : Cadena de caracteres en la que se tendrá que especificar la misma contraseña que se configuró en el servidor
WebLinkRestAPI para acceder a este servicio.

**TiempoSession** **_(integer)_** : Valor numérico con el que se podrá especificar la cantidad de minutos que debe durar la conexión entre
el cliente y el servicio WebLinkRestAPI. Como máximo se podrá establecer en 59 minutos.

**CodigoIntegrador (String):** Código suministrado por BdpSoftware que identifica a un integrador en nuestros sistemas. Sin este
código no se va a poder realizar peticiones a WeblinkRestAPI

##### JSON de solicitud

{
"Login": "test"
"Password": "12345678"
"TiempoSession": 59
"CodigoIntegrador": "ASDF"
}

##### Parámetros de salida

**ErrorMessage** **_(string)_** : Cadena que almacenará posibles errores. Si la consulta se realizó correctamente, estará vacía.

**_AuthSession(AuthSessionDataType):_** Objeto que contendrá los datos de autenticación de la sesión que se acaba de establecer entre
cliente y servidor. Estará compuesto por los siguientes campos:

```
Token(string): Cadena de texto que mostrará el token de autenticación de esta sesión.
```
```
ExpiresIN_InSecconds(integer): Valor numérico que mostrará la cantidad de segundos que durará la sesión que acaba de
establecerse entre cliente y servidor.
```
##### JSON de respuesta

{
"ErrorMessage": "",
"AuthSession": {
"Token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJqdGkiOiJjY _[...]_ EbX-8mQSSGw0lrrBzGPDE1gE9l-h4",
"ExpiresIn_InSecconds": 3540
}
}


## Categoría Artículos

En esta categoría se incluirán todos los comandos que permitirán interactuar con datos de artículos. Los comandos disponibles para
esta categoría serán los siguientes:

1. **GetArticle** : Retorna los datos de un artículo.
2. **GetPricesArticles** : Retorna los precios de venta (del 1 al 5) de un artículo.
3. **ExportArticles** : Retorna una lista de artículos aplicando unos filtros de selección.
4. **GetPOSArticlesList** : Devuelve los artículos de un perfil de departamentos y artículos.
5. **GetFullArticlesList** : Devuelve la información completa de los artículos de un perfil de departamentos y artículos.
6. **CreateArticlesAndUpdateProfiles** : Crea un artículo y, opcionalmente, lo añade a un conjunto de perfiles.
7. **ModifyPricesArticles** : Modifica los precios de venta y descuentos de un conjunto de artículos.
8. **ModifyArticleAndUpdateProfile** : Modifica un artículo y, opcionalmente, también el artículo en los perfiles.


### GetArticle

Comando que retorna los datos de un artículo a partir de su código.

##### Ruta de llamada

/API/Articles/Get

##### Parámetros de entrada

**ArtCode** **_(long)_** : Valor mayor que cero de un máximo de 13 dígitos el cual corresponderá al código del artículo del que se desea
obtener información.

##### JSON de solicitud

{
"ArtCode": 1001
}

##### Parámetros de salida

**ErrorMessage** **_(string)_** : Cadena que almacenará posibles errores. Si la consulta se realizó correctamente, estará vacía.

**ArticleData** **_(ArticleListDataType)_** : Objeto **ArticleListDataType** que contendrá los datos del artículo. Estará compuesto por los
siguientes elementos:

```
DeptCode (integer) : Valor con el código de departamento de venta asociado al artículo.
```
```
DeptDescription (string) : Cadena con la descripción del departamento de venta asociado al artículo.
```
```
MenuDish (boolean) : Campo lógico que corresponde con el campo ‘Plato de Menú’ del artículo.
```
```
WebArticle (boolean) : Campo lógico que corresponde con el campo ‘Artículo Web’ del artículo.
```
```
POS_SupplementsProfileID (decimal) : Valor con el código de perfil de suplementos asociado al artículo.
```
```
SelfOrdering_CommentsProfileID (decimal) : Valor con el código de perfil de comentarios asociado a la configuración de
autocomanda del artículo.
```
```
SelfOrdering_SupplementsProfileID (decimal) : Valor con el código de perfil de suplementos asociado a la configuración de
autocomanda del artículo.
```
```
POS_MenuID (decimal) : Valor con el código de definición de menú asociado al artículo.
```
```
POS_FastfoodID (decimal) : Valor con el código de definición fastfood asociado al artículo.
```
```
POS_PackID (decimal) : Valor con el código de definición de pack asociado al artículo.
```
```
Is_Inventoriable (boolean) : Campo lógico que corresponde con el campo ‘Inventariable’ del artículo.
```
```
BuyTAVCode (integer) : Valor con el código de IVA de compra asociado al artículo.
```

**BuyTAVPer** **_(decimal)_** : Valor decimal con el porcentaje del IVA de compra asociado al artículo.

**TAVCode** **_(integer)_** : Valor con el código de IVA de venta asociado al artículo.

**TAVPer** **_(decimal)_** : Valor decimal con el porcentaje del IVA de venta asociado al artículo.

**AuxPrinters** **_(string)_** : Cadena con la secuencia de impresoras auxiliares asociadas al artículo.

**Commissionable** **_(boolean)_** : Campo lógico que corresponde con el campo ‘Artículo Comisionable’ del artículo.

**ModifiablePrice** **_(boolean)_** : Campo lógico que corresponde con el campo ‘Precio Modificable’ del artículo.

**DontPrintTicketValue0** **_(boolean)_** : Campo lógico que corresponde con el campo ‘No Imprimir en Factura si Valor 0’ del
artículo.

**Weight** **_(boolean)_** : Campo lógico que corresponde con el campo ‘Artículo de Peso’ del artículo.

**DontNotifyUnitsPrice0** **_(boolean)_** : Campo lógico que corresponde con el campo ‘No Avisar si unidades o Precio es 0’ del
artículo.

**NotifyModifyPriceUnits** **_(boolean)_** : Campo lógico que corresponde con el campo ‘Avisar para Modificar Precio y/o
Unidades’ del artículo.

**TwoForOne** **_(boolean)_** : Campo lógico que corresponde con el campo ‘Artículo Dos x Uno” del artículo.

**POS_CommentsProfileID (decimal)** : Valor con el código de perfil de comentarios asociado al artículo.

**ErrorMessage** **_(string)_** : Cadena que almacenará posibles errores. Si la consulta se realizó correctamente, estará vacía.

**PriceConfirmation** **_(boolean)_** : Campo lógico que corresponde con el campo ‘Confirmación Precio’ del artículo.

**FreeDescription** **_(boolean)_** : Campo lógico que corresponde con el campo ‘Descripción Libre’ del artículo.

**IsCombinable** **_(boolean)_** : Campo lógico que indica si el artículo es combinable.

**CombinedDescription** **_(string)_** : Cadena de texto que corresponde a la descripción del combinado (máx. 15 caracteres).

**CombBasePrice1** **_(decimal)_** : Valor decimal del precio base 1 del combinado.

**CombBasePrice2** **_(decimal)_** : Valor decimal del precio base 2 del combinado.

**CombBasePrice3** **_(decimal)_** : Valor decimal del precio base 3 del combinado.

**CombBasePrice4** **_(decimal)_** : Valor decimal del precio base 4 del combinado.

**CombBasePrice5** **_(decimal)_** : Valor decimal del precio base 5 del combinado.

**CombAuxPrice1** **_(decimal)_** : Valor decimal del precio auxiliar 1 del combinado.

**CombAuxPrice2** **_(decimal)_** : Valor decimal del precio auxiliar 2 del combinado.

**CombAuxPrice3** **_(decimal)_** : Valor decimal del precio auxiliar 3 del combinado.

**CombAuxPrice4** **_(decimal)_** : Valor decimal del precio auxiliar 4 del combinado.

**CombAuxPrice5** **_(decimal)_** : Valor decimal del precio auxiliar 5 del combinado.


**ActivateAlwaysCombined** **_(boolean)_** : Campo lógico que corresponde al campo ‘Activar Siempre Combinado’.

**MandatoryCombined** **_(integer)_** : Valor entero que indica la Obligatoriedad de Combinado: 1: Obligatoriedad de Base; 2:
Obligatoriedad de Auxiliar; 3: Indiferente.

**CombinedAssocType** **_(integer)_** : Valor entero que corresponde al campo ‘Departamentos Asociados al Combinado’: 1: No
Asociar; 2: Asociar a un Departamento; 3: Asociar a una Maxi-Pantalla.

**CombinedDepartmentAssoc** **_(integer)_** : Valor entero que indica el código de Departamento Asociado a los combinados.

**CombinedDepartmentAssocDescription** **_(string)_** : Cadena de texto que corresponde a la descripción del Departamento
Asociado a los combinados (máx. 40 caracteres).

**CombinedMaxiscreenAssoc** **_(integer)_** : Valor entero que indica el código de Maxipantalla Asociada a los combinados.

**CombinedMaxiscreenAssocDescription** **_(string)_** : Cadena de texto que corresponde a la descripción de la Maxipantalla
Asociada a los combinados (máx. 40 caracteres).

**ApplyDiscountsInComb** **_(boolean)_** : Campo lógico que corresponde al campo ‘Aplicar Descuentos Generales del Artículo en
Combinados’.

**ArtDescription** **_(string)_** : Cadena de texto que corresponde a la descripción del artículo (máx. 40 caracteres).

**Price1** **_(decimal)_** : Valor decimal con el PVP1 del artículo.

**Price2** **_(decimal)_** : Valor decimal con el PVP2 del artículo.

**Price3** **_(decimal)_** : Valor decimal con el PVP3 del artículo.

**Price4** **_(decimal)_** : Valor decimal con el PVP4 del artículo.

**Price5** **_(decimal)_** : Valor decimal con el PVP5 del artículo.

**Dct1** **_(decimal)_** : Valor decimal con el porcentaje de descuento 1 del artículo.

**Dct2** **_(decimal)_** : Valor decimal con el porcentaje de descuento 2 del artículo.

**Dct3** **_(decimal)_** : Valor decimal con el porcentaje de descuento 3 del artículo.

**Dct4** **_(decimal)_** : Valor decimal con el porcentaje de descuento 4 del artículo.

**Dct5** **_(decimal)_** : Valor decimal con el porcentaje de descuento 5 del artículo.

**GraphDescrip1** **_(string)_** : Cadena de texto que corresponde a la Descripción Gráfica de la Línea 1 del artículo.

**GraphDescrip2** **_(string)_** : Cadena de texto que corresponde a la Descripción Gráfica de la Línea 2 del artículo.

**GraphDescrip3** **_(string)_** : Cadena de texto que corresponde a la Descripción Gráfica de la Línea 3 del artículo.

**ExtendedArtDescription** **_(string)_** : Cadena de texto que corresponde a la Descripción Extendida del artículo.

**Proportion1Description** **_(string)_** : Cadena de texto que corresponde al campo ‘Descripción de la 1ª Proporción’ (máx. 40
caracteres).

**Proportion2Active** **_(boolean)_** : Campo lógico que corresponde al campo ‘Activar 2ª Proporción’.

**Proportion3Active** **_(boolean)_** : Campo lógico que corresponde al campo ‘Activar 3ª Proporción’.


**Proportion4Active** **_(boolean)_** : Campo lógico que corresponde al campo ‘Activar 4ª Proporción’.

**Proportion5Active** **_(boolean)_** : Campo lógico que corresponde al campo ‘Activar 5ª Proporción’.

**Proportion6Active** **_(boolean)_** : Campo lógico que corresponde al campo ‘Activar 6ª Proporción’.

**Proportion7Active** **_(boolean)_** : Campo lógico que corresponde al campo ‘Activar 7ª Proporción’.

**Proportion8Active** **_(boolean)_** : Campo lógico que corresponde al campo ‘Activar 8ª Proporción’.

**Proportion9Active** **_(boolean)_** : Campo lógico que corresponde al campo ‘Activar 9ª Proporción’.

**Proportion2Amount** **_(decimal)_** : Valor decimal correspondiente al campo ‘Cantidad 2ª Proporción’.

**Proportion3Amount** **_(decimal)_** : Valor decimal correspondiente al campo ‘Cantidad 3ª Proporción’.

**Proportion4Amount** **_(decimal)_** : Valor decimal correspondiente al campo ‘Cantidad 4ª Proporción’.

**Proportion5Amount** **_(decimal)_** : Valor decimal correspondiente al campo ‘Cantidad 5ª Proporción’.

**Proportion6Amount** **_(decimal)_** : Valor decimal correspondiente al campo ‘Cantidad 6ª Proporción’.

**Proportion7Amount** **_(decimal)_** : Valor decimal correspondiente al campo ‘Cantidad 7ª Proporción’.

**Proportion8Amount** **_(decimal)_** : Valor decimal correspondiente al campo ‘Cantidad 8ª Proporción’.

**Proportion9Amount** **_(decimal)_** : Valor decimal correspondiente al campo ‘Cantidad 9ª Proporción’.

**Proportion2Description** **_(string)_** : Cadena de texto que corresponde al campo ‘Descripción de la 2ª Proporción’ (máx. 40
caracteres).

**Proportion3Description** **_(string)_** : Cadena de texto que corresponde al campo ‘Descripción de la 3ª Proporción’ (máx. 40
caracteres).

**Proportion4Description** **_(string)_** : Cadena de texto que corresponde al campo ‘Descripción de la 4ª Proporción’ (máx. 40
caracteres).

**Proportion5Description** **_(string)_** : Cadena de texto que corresponde al campo ‘Descripción de la 5ª Proporción’ (máx. 40
caracteres).

**Proportion6Description** **_(string)_** : Cadena de texto que corresponde al campo ‘Descripción de la 6ª Proporción’ (máx. 40
caracteres).

**Proportion7Description** **_(string)_** : Cadena de texto que corresponde al campo ‘Descripción de la 7ª Proporción’ (máx. 40
caracteres).

**Proportion8Description** **_(string)_** : Cadena de texto que corresponde al campo ‘Descripción de la 8ª Proporción’ (máx. 40
caracteres).

**Proportion9Description** **_(string)_** : Cadena de texto que corresponde al campo ‘Descripción de la 9ª Proporción’ (máx. 40
caracteres).

**Proportion2Price1** **_(decimal)_** : Valor decimal con el PVP1 de la 2ª Proporción.

**Proportion3Price1** **_(decimal)_** : Valor decimal con el PVP1 de la 3ª Proporción.

**Proportion4Price1** **_(decimal)_** : Valor decimal con el PVP1 de la 4ª Proporción.


**Proportion5Price1** **_(decimal)_** : Valor decimal con el PVP1 de la 5ª Proporción.

**Proportion6Price1** **_(decimal)_** : Valor decimal con el PVP1 de la 6ª Proporción.

**Proportion7Price1** **_(decimal)_** : Valor decimal con el PVP1 de la 7ª Proporción.

**Proportion8Price1** **_(decimal)_** : Valor decimal con el PVP1 de la 8ª Proporción.

**Proportion9Price1** **_(decimal)_** : Valor decimal con el PVP1 de la 9ª Proporción.

**Proportion2Price2** **_(decimal)_** : Valor decimal con el PVP2 de la 2ª Proporción.

**Proportion3Price2** **_(decimal)_** : Valor decimal con el PVP2 de la 3ª Proporción.

**Proportion4Price2** **_(decimal)_** : Valor decimal con el PVP2 de la 4ª Proporción.

**Proportion5Price2** **_(decimal)_** : Valor decimal con el PVP2 de la 5ª Proporción.

**Proportion6Price2** **_(decimal)_** : Valor decimal con el PVP2 de la 6ª Proporción.

**Proportion7Price2** **_(decimal)_** : Valor decimal con el PVP2 de la 7ª Proporción.

**Proportion8Price2** **_(decimal)_** : Valor decimal con el PVP2 de la 8ª Proporción.

**Proportion9Price2** **_(decimal)_** : Valor decimal con el PVP2 de la 9ª Proporción.

**Proportion2Price3** **_(decimal)_** : Valor decimal con el PVP3 de la 2ª Proporción.

**Proportion3Price3** **_(decimal)_** : Valor decimal con el PVP3 de la 3ª Proporción.

**Proportion4Price3** **_(decimal)_** : Valor decimal con el PVP3 de la 4ª Proporción.

**Proportion5Price3** **_(decimal)_** : Valor decimal con el PVP3 de la 5ª Proporción.

**Proportion6Price3** **_(decimal)_** : Valor decimal con el PVP3 de la 6ª Proporción.

**Proportion7Price3** **_(decimal)_** : Valor decimal con el PVP3 de la 7ª Proporción.

**Proportion8Price3** **_(decimal)_** : Valor decimal con el PVP3 de la 8ª Proporción.

**Proportion9Price3** **_(decimal)_** : Valor decimal con el PVP3 de la 9ª Proporción.

**Proportion2Price4** **_(decimal)_** : Valor decimal con el PVP4 de la 2ª Proporción.

**Proportion3Price4** **_(decimal)_** : Valor decimal con el PVP4 de la 3ª Proporción.

**Proportion4Price4** **_(decimal)_** : Valor decimal con el PVP4 de la 4ª Proporción.

**Proportion5Price4** **_(decimal)_** : Valor decimal con el PVP4 de la 5ª Proporción.

**Proportion6Price4** **_(decimal)_** : Valor decimal con el PVP4 de la 6ª Proporción.

**Proportion7Price4** **_(decimal)_** : Valor decimal con el PVP4 de la 7ª Proporción.

**Proportion8Price4** **_(decimal)_** : Valor decimal con el PVP4 de la 8ª Proporción.

**Proportion9Price4** **_(decimal)_** : Valor decimal con el PVP4 de la 9ª Proporción.


**Proportion2Price5** **_(decimal)_** : Valor decimal con el PVP5 de la 2ª Proporción.

**Proportion3Price5** **_(decimal)_** : Valor decimal con el PVP5 de la 3ª Proporción.

**Proportion4Price5** **_(decimal)_** : Valor decimal con el PVP5 de la 4ª Proporción.

**Proportion5Price5** **_(decimal)_** : Valor decimal con el PVP5 de la 5ª Proporción.

**Proportion6Price5** **_(decimal)_** : Valor decimal con el PVP5 de la 6ª Proporción.

**Proportion7Price5** **_(decimal)_** : Valor decimal con el PVP5 de la 7ª Proporción.

**Proportion8Price5** **_(decimal)_** : Valor decimal con el PVP5 de la 8ª Proporción.

**Proportion9Price5** **_(decimal)_** : Valor decimal con el PVP5 de la 9ª Proporción.

**Proportion2PluDiscount** **_(integer)_** : Valor entero que corresponde con el campo ‘PLU Descontar de la 2ª Proporción’.

**Proportion2PluDiscountDescription** **_(string)_** : Cadena de texto que corresponde al campo de la descripción del ‘PLU
Descontar de la 2ª Proporción’.

**Proportion3PluDiscount** **_(integer)_** : Valor entero que corresponde con el campo ‘PLU Descontar de la 3ª Proporción’.

**Proportion3PluDiscountDescription** **_(string)_** : Cadena de texto que corresponde al campo de la descripción del ‘PLU
Descontar de la 3ª Proporción’.

**Proportion4PluDiscount** **_(integer)_** : Valor entero que corresponde con el campo ‘PLU Descontar de la 4ª Proporción’.

**Proportion4PluDiscountDescription** **_(string)_** : Cadena de texto que corresponde al campo de la descripción del ‘PLU
Descontar de la 4ª Proporción’.

**Proportion5PluDiscount** **_(integer)_** : Valor entero que corresponde con el campo ‘PLU Descontar de la 5ª Proporción’.

**Proportion5PluDiscountDescription** **_(string)_** : Cadena de texto que corresponde al campo de la descripción del ‘PLU
Descontar de la 5ª Proporción’.

**Proportion6PluDiscount** **_(integer)_** : Valor entero que corresponde con el campo ‘PLU Descontar de la 6ª Proporción’.

**Proportion6PluDiscountDescription** **_(string)_** : Cadena de texto que corresponde al campo de la descripción del ‘PLU
Descontar de la 6ª Proporción’.

**Proportion7PluDiscount** **_(integer)_** : Valor entero que corresponde con el campo ‘PLU Descontar de la 7ª Proporción’.

**Proportion7PluDiscountDescription** **_(string)_** : Cadena de texto que corresponde al campo de la descripción del ‘PLU
Descontar de la 7ª Proporción’.

**Proportion8PluDiscount** **_(integer)_** : Valor entero que corresponde con el campo ‘PLU Descontar de la 8ª Proporción’.

**Proportion8PluDiscountDescription** **_(string)_** : Cadena de texto que corresponde al campo de la descripción del ‘PLU
Descontar de la 8ª Proporción’.

**Proportion9PluDiscount** **_(integer)_** : Valor entero que corresponde con el campo ‘PLU Descontar de la 9ª Proporción’.

**Proportion9PluDiscountDescription** **_(string)_** : Cadena de texto que corresponde al campo de la descripción del ‘PLU
Descontar de la 9ª Proporción’.


```
ApplyDiscountsInProp (boolean) : campo lógico que corresponde al campo ‘Aplicar Descuentos Generales del Artículo en
Proporciones’.
```
```
NotAllowedAsInvitation (boolean) : campo lógico que corresponde al campo ‘No permitir cómo invitación’.
```
```
ArtCode (long) : Valor numérico de entre 1 y 13 dígitos correspondiente al código de del artículo.
```
##### JSON de respuesta

{
"ErrorMessage": "",
"ArticleData": {
"DeptCode": 1,
"DeptDescription": "REFRESCOS",
"MenuDish": true,
"WebArticle": true,
"POS_SupplementsProfileID": 0.0,
"SelfOrdering_CommentsProfileID": 0.0,
"SelfOrdering_SupplementsProfileID": 0.0,
"POS_MenuID": 0.0,
"POS_FastfoodID": 0.0,
"POS_PackID": 0.0,
"Is_Inventoriable": true,
"BuyTAVCode": 2,
"BuyTAVPer": 21.0,
"TAVCode": 1,
"TAVPer": 10.0,
"AuxPrinters": "",
"Commissionable": false,
"ModifiablePrice": true,
"DontPrintTicketValue0": false,
"Weight": false,
"DontNotifyUnitsPrice0": false,
"NotifyModifyPriceUnits": false,
"TwoForOne": false,
"POS_CommentsProfileID": 0.0,
"ErrorMessage": "",
"PriceConfirmation": false,
"FreeDescription": false,
"IsCombinable": true,
"CombinedDescription": "COCACOLA",
"CombBasePrice1": 0.0,
"CombBasePrice2": 0.0,
"CombBasePrice3": 0.0,
"CombBasePrice4": 0.0,
"CombBasePrice5": 0.0,
"CombAuxPrice1": 0.6,
"CombAuxPrice2": 0.6,
"CombAuxPrice3": 0.6,
"CombAuxPrice4": 0.6,
"CombAuxPrice5": 0.6,
"ActivateAlwaysCombined": false,
"MandatoryCombined": 2,
"CombinedAssocType": 1,
"CombinedDepartmentAssoc": 0,
"CombinedDepartmentAssocDescription": "",
"CombinedMaxiscreenAssoc": 0,
"CombinedMaxiscreenAssocDescription": "",
"ApplyDiscountsInComb": false,
"ArtDescription": "COCA-COLA",
"Price1": 1.05,
"Price2": 1.05,
"Price3": 1.05,


"Price4": 1.05,
"Price5": 1.05,
"Dct1": 0.0,
"Dct2": 0.0,
"Dct3": 0.0,
"Dct4": 0.0,
"Dct5": 0.0,
"GraphDescrip1": "",
"GraphDescrip2": "",
"GraphDescrip3": "",
"ExtendedArtDescription": "",
"Proportion1Description": "",
"Proportion2Active": false,
"Proportion3Active": false,
"Proportion4Active": false,
"Proportion5Active": false,
"Proportion6Active": false,
"Proportion7Active": false,
"Proportion8Active": false,
"Proportion9Active": false,
"Proportion2Amount": 0.0,
"Proportion3Amount": 0.0,
"Proportion4Amount": 0.0,
"Proportion5Amount": 0.0,
"Proportion6Amount": 0.0,
"Proportion7Amount": 0.0,
"Proportion8Amount": 0.0,
"Proportion9Amount": 0.0,
"Proportion2Description": "",
"Proportion3Description": "",
"Proportion4Description": "",
"Proportion5Description": "",
"Proportion6Description": "",
"Proportion7Description": "",
"Proportion8Description": "",
"Proportion9Description": "",
"Proportion2Price1": 0.0,
"Proportion3Price1": 0.0,
"Proportion4Price1": 0.0,
"Proportion5Price1": 0.0,
"Proportion6Price1": 0.0,
"Proportion7Price1": 0.0,
"Proportion8Price1": 0.0,
"Proportion9Price1": 0.0,
"Proportion2Price2": 0.0,
"Proportion3Price2": 0.0,
"Proportion4Price2": 0.0,
"Proportion5Price2": 0.0,
"Proportion6Price2": 0.0,
"Proportion7Price2": 0.0,
"Proportion8Price2": 0.0,
"Proportion9Price2": 0.0,
"Proportion2Price3": 0.0,
"Proportion3Price3": 0.0,
"Proportion4Price3": 0.0,
"Proportion5Price3": 0.0,
"Proportion6Price3": 0.0,
"Proportion7Price3": 0.0,
"Proportion8Price3": 0.0,
"Proportion9Price3": 0.0,
"Proportion2Price4": 0.0,
"Proportion3Price4": 0.0,
"Proportion4Price4": 0.0,
"Proportion5Price4": 0.0,
"Proportion6Price4": 0.0,
"Proportion7Price4": 0.0,


"Proportion8Price4": 0.0,
"Proportion9Price4": 0.0,
"Proportion2Price5": 0.0,
"Proportion3Price5": 0.0,
"Proportion4Price5": 0.0,
"Proportion5Price5": 0.0,
"Proportion6Price5": 0.0,
"Proportion7Price5": 0.0,
"Proportion8Price5": 0.0,
"Proportion9Price5": 0.0,
"Proportion2PluDiscount": 0,
"Proportion2PluDiscountDescription": null,
"Proportion3PluDiscount": 0,
"Proportion3PluDiscountDescription": null,
"Proportion4PluDiscount": 0,
"Proportion4PluDiscountDescription": null,
"Proportion5PluDiscount": 0,
"Proportion5PluDiscountDescription": null,
"Proportion6PluDiscount": 0,
"Proportion6PluDiscountDescription": null,
"Proportion7PluDiscount": 0,
"Proportion7PluDiscountDescription": null,
"Proportion8PluDiscount": 0,
"Proportion8PluDiscountDescription": null,
"Proportion9PluDiscount": 0,
"Proportion9PluDiscountDescription": null,
"ApplyDiscountsInProp": false,
"ArtCode": 1001
}
}


### GetPricesArticles

Comando que retorna los precios de venta (del 1 al 5) de un artículo a partir de su código.

##### Ruta de llamada

/API/Articles/GetPrices

##### Parámetros de entrada

**ArtCode** **_(long)_** : Valor mayor que cero de un máximo de 13 dígitos el cual corresponderá al código del artículo del que se desea
obtener información.

##### JSON de solicitud

{
"ArtCode": 1001
}

##### Parámetros de salida

**Prices** **_(List decimal)_** : Lista de decimales que contendrá los precios (del 1 al 5) del artículo.

**Disconts** **_(List decimal)_** : Lista de decimales que contendrá los descuentos a aplicar a los precios (del 1 al 5) del artículo.

**ErrorMessage** **_(string)_** : Cadena que almacenará posibles errores. Si la consulta se realizó correctamente, estará vacía.

##### JSON de respuesta

{
"Prices": [
1.05,
1.05,
1.05,
1.05,
1.05
],
"Discounts": [
25.0,
20.0,
15.0,
10.0,
0.0
],
"ErrorMessage": ""
}


### ExportArticles

Comando que retorna una exportación de artículos en una lista con los datos de varios artículos aplicando unos filtros de selección.
En esta exportación tan sólo se incluirán artículos que en su ficha tengan marca la opción de “Artículo Web” (en la pestaña Web del
mantenimiento de artículos). Se podrá especificar un intervalo de departamentos y artículos para filtrar la exportación. También se
podrá filtrar la exportación a artículos con el parámetro de modificado marcado. El nivel precio (de los 5 disponibles en la aplicación)
que se exporte para los artículos corresponderá al especificado en la consulta. Para los artículos de talla y color se podrá exportar el
precio de rebaja. El nivel precio de rebaja (de los 3 disponibles en la aplicación) que se exporte para los artículos de talla y color
corresponderá al especificado en la consulta.

##### Ruta de llamada

/API/Articles/Export

##### Parámetros de entrada

**Dept1** **_(integer)_** : Valor entero de 1 a 3 dígitos mayor que cero correspondiente al código inicial del departamento de venta de los
artículos que se van a exportar.

**Dept2** **_(integer)_** : Valor entero de 1 a 3 dígitos mayor que cero correspondiente al código final del departamento de venta de los
artículos que se van a exportar.

**Art1** **_(long)_** : Valor entero de 1 a 13 dígitos mayor que cero correspondiente al código inicial del intervalo de artículos que se van a
exportar.

**Art2** **_(long)_** : Valor entero de 1 a 13 dígitos mayor que cero correspondiente al código final del intervalo de artículos que se van a
exportar.

**Modified** **_(boolean)_** : Campo lógico. Si se asigna a ‘True’, en la exportación tan sólo se incluirán artículos del intervalo seleccionado
que hayan sido modificados. Si se asigna a ‘False’, se exportarán todos los artículos del intervalo seleccionado.

**TypePrice** **_(integer)_** : valor entero entre 1 y 5 que permitirá especificar el nivel de precios (del 1 al 5) que se devolverá para los artículos
en esta exportación.

**Disc** **_(integer)_** : Valor válido par artículos de talla y color (0 si no es una aplicación de talla y color). Entero entre 0 y 3 que indica el
tipo de precio de rebaja que se devolverá para los artículos en la exportación.

##### JSON de solicitud

{
"Dept1": 1,
"Dept2": 999,
"Art1": 1,
"Art2": 9999999999999,
"Modified": false,
"TypePrice": 1,
"Disc": 0
}

##### Parámetros de salida

**ErrorMessage** **_(string)_** : Cadena que almacenará posibles errores. Si la consulta se realizó correctamente, estará vacía.


**ArticleData** **_(InfoArticle)_** : Colección de objetos **InfoArticle** que contendrá los datos de los artículos exportados. Cada objeto
**InfoArticle** corresponderá a un artículo y estará compuesto por los siguientes elementos:

```
Article (long) : Valor numérico de entre 1 y 13 dígitos correspondiente al código de del artículo.
```
```
Description (string) : Cadena de texto que corresponde a la descripción del artículo (máx. 40 caracteres).
```
```
PLUAddress (string) : Cadena de texto que contendrá el path relativo (dentro de la carpeta imágenes de la aplicación) del
disco duro en el que se almacena el gráfico asignado al artículo (por ejemplo “@\NombreImagen.bmp”).
Price (decimal) : Valor decimal con el precio correspondiente al nivel seleccionado del artículo cuando el artículo no
corresponde a uno de talla y color.
```
```
Discount (decimal) : Valor decimal con el porcentaje de descuento correspondiente al nivel seleccionado del artículo cuando
el artículo no corresponde a uno de talla y color.
```
```
PriceDisc (decimal) : Valor decimal que indicará el precio de rebaja general (no el de las líneas de precios
de talla y color) correspondiente al nivel seleccionado del artículo si la aplicación corresponde a la de talla y color. Si no es
una aplicación de talla y color este valor será 0.
```
```
PerDisc (decimal) : Valor decimal que indicará el porcentaje de rebaja general (no el de las líneas de
precios de talla y color) correspondiente al nivel seleccionado del artículo si la aplicación corresponde a la de talla y color.
Si no es una aplicación de talla y color este valor será 0.
```
```
SalesVAT (decimal) : Valor decimal con el porcentaje del IVA de venta asociado al artículo.
```
```
SAC (boolean) : Campo lógico que en caso de ser True indicará que el artículo en cuestión es de talla y color y en caso de
ser false, indicará que se no se trata de un artículo de talla y color.
```
```
Name_D1 (string) : Cadena de texto para artículos de talla y color. Indicará el nombre de la primera
Dimensión del artículo (por ejemplo, TALLA).
```
```
Name_D2 (string) : Cadena de texto para artículos de talla y color. Indicará el nombre de la segunda Dimensión
del artículo (por ejemplo, COLOR).
```
```
Name_D3 (string) : Cadena de texto para artículos de talla y color. Indicará el nombre de la tercera Dimensión
del artículo (por ejemplo, COPA).
```
```
InfoLines List (InfoLinesArticle) : Colección de líneas que sólo estará disponible si el artículo corresponde a uno de talla y
color. Si no corresponde a uno de talla y color, este campo estará vacío. Cada objeto InfoLinesArticle contendrá información
de cada una de las dimensiones y estará compuesto por los siguientes elementos:
```
```
D1 (string) : Cadena de texto que mostrará el contenido de la primera dimensión del artículo. Si, por ejemplo,
Name_D1 es “TALLA”, aquí iría el número de talla (por ejemplo: 45).
```
```
D2 (string) : Cadena de texto que mostrará el contenido de la segunda dimensión del artículo. Si, por ejemplo,
Name_D2 es “COLOR”, aquí iría el nombre del color (por ejemplo: AZUL).
```
```
D3 (string) : Cadena de texto que mostrará el contenido de la tercera dimensión del artículo. Si, por ejemplo,
Name_D3 es “COPA” aquí iría el tipo de copa (ejemplo: A).
```
```
KeyWords (string) : Cadena de texto de 250 caracteres que mostrará el campo KeyWords asignado al artículo en cuestión
(en la pestaña Web del mantenimiento de artículos).
```
```
Outlet (boolean) : Campo lógico que corresponde con el campo ‘Producto Outlet’ del artículo (en la pestaña Web del
mantenimiento de artículos).
```
```
Is_Inventoriable (boolean) : Campo lógico que corresponde con el campo ‘Inventariable’ del artículo.
```

```
BuyTAVCode (integer) : Valor con el código de IVA de compra asociado al artículo.
```
```
BuyTAVPer (decimal) : Valor decimal con el porcentaje del IVA de compra asociado al artículo.
```
```
ErrorMessage (string) : Cadena que almacenará posibles errores. Si la consulta se realizó correctamente, estará vacía.
```
##### JSON de respuesta

{
"ErrorMessage": "",
"Articles": [
{
"Article": 1001,
"Description": "COCA-COLA",
"PLUAddress": "@\\Plus\\CocaCola.bmp",
"Price": 1.0,
"Discount": 25.0,
"PriceDisc": 0.0,
"PerDisc": 0.0,
"SalesVAT": 10.0,
"SAC": false,
"Name_D1": null,
"Name_D2": null,
"Name_D3": null,
"InfoLines": [],
"KeyWords": "",
"Outlet": false,
"Is_Inventoriable": true,
"BuyTAVCode": 1,
"BuyTAVPer": 10.0,
"ErrorMessage": null
},
{
"Article": 1002,
"Description": "SCHWEPPES LIMÓN",
"PLUAddress": "@\\Plus\\SchweppesLimon.bmp",
"Price": 1.05,
"Discount": 0.0,
"PriceDisc": 0.0,
"PerDisc": 0.0,
"SalesVAT": 4.0,
"SAC": false,
"Name_D1": null,
"Name_D2": null,
"Name_D3": null,
"InfoLines": [],
"KeyWords": "",
"Outlet": false,
"Is_Inventoriable": true,
"BuyTAVCode": 2,
"BuyTAVPer": 21.0,
"ErrorMessage": null
}
]
}


### GetPOSArticlesList

Obtiene una lista con la información de los artículos de un perfil de departamentos y artículos determinado. Los campos que no
existan en la ficha de los artículos en perfil, se cogerán de la tabla de artículos.

##### Ruta de llamada

/API/Articles/GetPOSList

##### Parámetros de entrada

**Art1** **_(long)_** : Valor entero de 1 a 13 dígitos mayor que cero correspondiente al código inicial del intervalo de artículos que se van a
exportar.

**Art2** **_(long)_** : Valor entero de 1 a 13 dígitos mayor que cero correspondiente al código final del intervalo de artículos que se van a
exportar.

**Dept1** **_(integer)_** : Valor entero de 1 a 3 dígitos mayor que cero correspondiente al código inicial del departamento de venta de los
artículos que se van a exportar.

**Dept2** **_(integer)_** : Valor entero de 1 a 3 dígitos mayor que cero correspondiente al código final del departamento de venta de los
artículos que se van a exportar.

**Description** **_(string)_** : Cadena de texto que permite especificar una cadena de texto. Esta cadena de texto será utilizada para filtrar la
exportación de artículos aplicando el filtro a la descripción del departamento del artículo en cuestión.

**DescriptionQueryType** **_(StringSearchType)_** : Campo te tipo numérico que permitirá especificar el tipo de búsqueda que se realizará
en la descripción del departamento de los artículos para aplicar el filtrado. Los tipos disponibles son:

```
None = 0 : Ningún tipo. No se aplicará filtrado por descripción de departamento.
Starts = 1 : que empiece por. Se filtrará la exportación de artículos a artículos que su departamento empiece por la cadena
especificada en el campo Description.
Contains = 2 : que contenga. Se filtrará la exportación de artículos a artículos que su departamento contenga la cadena
especificada en el campo Description.
Ends = 3 : que acabe en. Se filtrará la exportación de artículos a artículos que su departamento acabe en la cadena
especificada en el campo Description.
```
**ItemsPerPage(** **_integer_** **)** : Valor numérico que permitirá que los resultados de datos de artículos sean paginados. Mediante este valor
se puede especificar la cantidad de elementos de artículos que contendrá cada página.

**ActualPage(** **_integer_** **)** : Valor numérico que presentará utilidad si se dan los resultados paginados. Permitirá especificar la página
actual.

**nField(** **_GetDepartmentField_** **)** : Campo de tipo numérico que permitirá especificar el campo por el que se deberán ordenar los
resultados. Los tipos de ordenación disponibles son:

```
ArtCode = 1, ArtDescription = 2, Price1 = 3, Price2 = 4, Price3 = 5, Price4 = 6, Price5 = 7, Dct1 = 8, Dct2 = 9, Dct3 = 10, Dct4
= 11, Dct5 = 12, DeptCode = 13, TAVCode = 15, GraphDescrip1 = 17, GraphDescrip2 = 18, GraphDescrip3 = 19.
```
**nOrder(** **_OrderType_** **)** : Campo de tipo numérico que permitirá especificar el tipo de ordenación a aplicar a la ordenación especificada
en el campo anterior que se aplicará a los resultados. Los tipos de ordenación disponibles son:

```
ASC = 0 : ascendente (de menor a mayor).
```
```
DESC = 1 : descendente (de mayor a menor).
```

**ProfileCode:** Valor numérico que permitirá especificar el código de perfil de artículos para el que se va a obtener sus artículos.

##### JSON de solicitud

###### {

```
"Art1": 0,
"Art2": 9999999999999,
"Dept1": 0,
"Dept2": 999,
"Description": "",
"DescriptionQueryType": 0,
"ItemsPerPage": 10,
"ActualPage": 1,
"nField": 1,
"nOrder": 0,
"ProfileCode": 2
}
```
##### Parámetros de salida

**NumberItems:** Valor numérico que indicará la cantidad de artículos obtenidos con esta consulta.

**ErrorMessage** **_(string)_** : Cadena que almacenará posibles errores. Si la consulta se realizó correctamente, estará vacía.

**ArticleListData** **_(ArticleListDataType)_** : Lista de objetos **_ArticleListDataType_** que contendrá los datos de los artículos obtenidos. Cada
objeto **_ArticleListDataType_** corresponderá a un artículo y estará compuesto por los siguientes elementos:

```
DeptCode (integer) : Valor con el código de departamento de venta asociado al artículo.
```
```
DeptDescription (string) : Cadena con la descripción del departamento de venta asociado al artículo.
```
```
MenuDish (boolean) : Campo lógico que corresponde con el campo ‘Plato de Menú’ del artículo.
```
```
WebArticle (boolean) : Campo lógico que corresponde con el campo ‘Artículo Web’ del artículo.
```
```
POS_SupplementsProfileID (decimal) : Valor con el código de perfil de suplementos asociado al artículo.
```
```
SelfOrdering_CommentsProfileID (decimal) : Valor con el código de perfil de comentarios asociado a la configuración de
autocomanda del artículo.
```
```
SelfOrdering_SupplementsProfileID (decimal) : Valor con el código de perfil de suplementos asociado a la configuración de
autocomanda del artículo.
```
```
POS_MenuID (decimal) : Valor con el código de definición de menú asociado al artículo.
```
```
POS_FastfoodID (decimal) : Valor con el código de definición fastfood asociado al artículo.
```
```
POS_PackID (decimal) : Valor con el código de definición de pack asociado al artículo.
```
```
Is_Inventoriable (boolean) : Campo lógico que corresponde con el campo ‘Inventariable’ del artículo.
```
```
BuyTAVCode (integer) : Valor con el código de IVA de compra asociado al artículo.
```
```
BuyTAVPer (decimal) : Valor decimal con el porcentaje del IVA de compra asociado al artículo.
```

**TAVCode** **_(integer)_** : Valor con el código de IVA de venta asociado al artículo.

**TAVPer** **_(decimal)_** : Valor decimal con el porcentaje del IVA de venta asociado al artículo.

**AuxPrinters** **_(string)_** : Cadena con la secuencia de impresoras auxiliares asociadas al artículo.

**Commissionable** **_(boolean)_** : Campo lógico que corresponde con el campo ‘Artículo Comisionable’ del artículo.

**ModifiablePrice** **_(boolean)_** : Campo lógico que corresponde con el campo ‘Precio Modificable’ del artículo.

**DontPrintTicketValue0** **_(boolean)_** : Campo lógico que corresponde con el campo ‘No Imprimir en Factura si Valor 0’ del
artículo.

**Weight** **_(boolean)_** : Campo lógico que corresponde con el campo ‘Artículo de Peso’ del artículo.

**DontNotifyUnitsPrice0** **_(boolean)_** : Campo lógico que corresponde con el campo ‘No Avisar si unidades o Precio es 0’.

**NotifyModifyPriceUnits** **_(boolean)_** : Campo lógico que corresponde con el campo ‘Avisar para Modificar Precio y/o
Unidades’ del artículo.

**TwoForOne** **_(boolean)_** : Campo lógico que corresponde con el campo ‘Artículo Dos x Uno” del artículo.

**POS_CommentsProfileID (decimal)** : Valor con el código de perfil de comentarios asociado al artículo.

**ErrorMessage** **_(string)_** : Cadena que almacenará posibles errores. Si la consulta se realizó correctamente, estará vacía.

**PriceConfirmation** **_(boolean)_** : Campo lógico que corresponde con el campo ‘Confirmación Precio’ del artículo.

**FreeDescription** **_(boolean)_** : Campo lógico que corresponde con el campo ‘Descripción Libre’ del artículo.

**IsCombinable** **_(boolean)_** : Campo lógico que indica si el artículo es combinable.

**CombinedDescription** **_(string)_** : Cadena de texto que corresponde a la descripción del combinado (máx. 15 caracteres).

**CombBasePrice1** **_(decimal)_** : Valor decimal del precio base 1 del combinado.

**CombBasePrice2** **_(decimal)_** : Valor decimal del precio base 2 del combinado.

**CombBasePrice3** **_(decimal)_** : Valor decimal del precio base 3 del combinado.

**CombBasePrice4** **_(decimal)_** : Valor decimal del precio base 4 del combinado.

**CombBasePrice5** **_(decimal)_** : Valor decimal del precio base 5 del combinado.

**CombAuxPrice1** **_(decimal)_** : Valor decimal del precio auxiliar 1 del combinado.

**CombAuxPrice2** **_(decimal)_** : Valor decimal del precio auxiliar 2 del combinado.

**CombAuxPrice3** **_(decimal)_** : Valor decimal del precio auxiliar 3 del combinado.

**CombAuxPrice4** **_(decimal)_** : Valor decimal del precio auxiliar 4 del combinado.

**CombAuxPrice5** **_(decimal)_** : Valor decimal del precio auxiliar 5 del combinado.

**ActivateAlwaysCombined** **_(boolean)_** : Campo lógico que corresponde al campo ‘Activar Siempre Combinado’.


**MandatoryCombined** **_(integer)_** : Valor entero que indica la Obligatoriedad de Combinado: 1: Obligatoriedad de Base; 2:
Obligatoriedad de Auxiliar; 3: Indiferente.

**CombinedAssocType** **_(integer)_** : Valor entero que corresponde al campo ‘Departamentos Asociados al Combinado’: 1: No
Asociar; 2: Asociar a un Departamento; 3: Asociar a una Maxi-Pantalla.

**CombinedDepartmentAssoc** **_(integer)_** : Valor entero que indica el código de Departamento Asociado a los combinados.

**CombinedDepartmentAssocDescription** **_(string)_** : Cadena de texto que corresponde a la descripción del Departamento
Asociado a los combinados (máx. 40 caracteres).

**CombinedMaxiscreenAssoc** **_(integer)_** : Valor entero que indica el código de Maxipantalla Asociada a los combinados.

**CombinedMaxiscreenAssocDescription** **_(string)_** : Cadena de texto que corresponde a la descripción de la Maxipantalla
Asociada a los combinados (máx. 40 caracteres).

**ApplyDiscountsInComb** **_(boolean)_** : Campo lógico que corresponde al campo ‘Aplicar Descuentos Generales del Artículo en
Combinados’.

**ArtDescription** **_(string)_** : Cadena de texto que corresponde a la descripción del artículo (máx. 40 caracteres).

**Price1** **_(decimal)_** : Valor decimal con el PVP1 del artículo.

**Price2** **_(decimal)_** : Valor decimal con el PVP2 del artículo.

**Price3** **_(decimal)_** : Valor decimal con el PVP3 del artículo.

**Price4** **_(decimal)_** : Valor decimal con el PVP4 del artículo.

**Price5** **_(decimal)_** : Valor decimal con el PVP5 del artículo.

**Dct1** **_(decimal)_** : Valor decimal con el porcentaje de descuento 1 del artículo.

**Dct2** **_(decimal)_** : Valor decimal con el porcentaje de descuento 2 del artículo.

**Dct3** **_(decimal)_** : Valor decimal con el porcentaje de descuento 3 del artículo.

**Dct4** **_(decimal)_** : Valor decimal con el porcentaje de descuento 4 del artículo.

**Dct5** **_(decimal)_** : Valor decimal con el porcentaje de descuento 5 del artículo.

**GraphDescrip1** **_(string)_** : Cadena de texto que corresponde a la Descripción Gráfica de la Línea 1 del artículo.

**GraphDescrip2** **_(string)_** : Cadena de texto que corresponde a la Descripción Gráfica de la Línea 2 del artículo.

**GraphDescrip3** **_(string)_** : Cadena de texto que corresponde a la Descripción Gráfica de la Línea 3 del artículo.

**ExtendedArtDescription** **_(string)_** : Cadena de texto que corresponde a la Descripción Extendida del artículo.

**Proportion1Description** **_(string)_** : Cadena de texto que corresponde al campo ‘Descripción de la 1ª Proporción’ (máx. 40
caracteres).

**Proportion2Active** **_(boolean)_** : Campo lógico que corresponde al campo ‘Activar 2ª Proporción’.

**Proportion3Active** **_(boolean)_** : Campo lógico que corresponde al campo ‘Activar 3ª Proporción’.

**Proportion4Active** **_(boolean)_** : Campo lógico que corresponde al campo ‘Activar 4ª Proporción’.


**Proportion5Active** **_(boolean)_** : Campo lógico que corresponde al campo ‘Activar 5ª Proporción’.

**Proportion6Active** **_(boolean)_** : Campo lógico que corresponde al campo ‘Activar 6ª Proporción’.

**Proportion7Active** **_(boolean)_** : Campo lógico que corresponde al campo ‘Activar 7ª Proporción’.

**Proportion8Active** **_(boolean)_** : Campo lógico que corresponde al campo ‘Activar 8ª Proporción’.

**Proportion9Active** **_(boolean)_** : Campo lógico que corresponde al campo ‘Activar 9ª Proporción’.

**Proportion2Amount** **_(decimal)_** : Valor decimal correspondiente al campo ‘Cantidad 2ª Proporción’.

**Proportion3Amount** **_(decimal)_** : Valor decimal correspondiente al campo ‘Cantidad 3ª Proporción’.

**Proportion4Amount** **_(decimal)_** : Valor decimal correspondiente al campo ‘Cantidad 4ª Proporción’.

**Proportion5Amount** **_(decimal)_** : Valor decimal correspondiente al campo ‘Cantidad 5ª Proporción’.

**Proportion6Amount** **_(decimal)_** : Valor decimal correspondiente al campo ‘Cantidad 6ª Proporción’.

**Proportion7Amount** **_(decimal)_** : Valor decimal correspondiente al campo ‘Cantidad 7ª Proporción’.

**Proportion8Amount** **_(decimal)_** : Valor decimal correspondiente al campo ‘Cantidad 8ª Proporción’.

**Proportion9Amount** **_(decimal)_** : Valor decimal correspondiente al campo ‘Cantidad 9ª Proporción’.

**Proportion2Description** **_(string)_** : Cadena de texto que corresponde al campo ‘Descripción de la 2ª Proporción’ (máx. 40
caracteres).

**Proportion3Description** **_(string)_** : Cadena de texto que corresponde al campo ‘Descripción de la 3ª Proporción’ (máx. 40
caracteres).

**Proportion4Description** **_(string)_** : Cadena de texto que corresponde al campo ‘Descripción de la 4ª Proporción’ (máx. 40
caracteres).

**Proportion5Description** **_(string)_** : Cadena de texto que corresponde al campo ‘Descripción de la 5ª Proporción’ (máx. 40
caracteres).

**Proportion6Description** **_(string)_** : Cadena de texto que corresponde al campo ‘Descripción de la 6ª Proporción’ (máx. 40
caracteres).

**Proportion7Description** **_(string)_** : Cadena de texto que corresponde al campo ‘Descripción de la 7ª Proporción’ (máx. 40
caracteres).

**Proportion8Description** **_(string)_** : Cadena de texto que corresponde al campo ‘Descripción de la 8ª Proporción’ (máx. 40
caracteres).

##### Proportion9Description (string) : Cadena de texto que corresponde al campo ‘Descripción de la 9ª Proporción’

##### (máx. 40 caracteres).

**Proportion2Price1** **_(decimal)_** : Valor decimal con el PVP1 de la 2ª Proporción.

**Proportion3Price1** **_(decimal)_** : Valor decimal con el PVP1 de la 3ª Proporción.

**Proportion4Price1** **_(decimal)_** : Valor decimal con el PVP1 de la 4ª Proporción.

**Proportion5Price1** **_(decimal)_** : Valor decimal con el PVP1 de la 5ª Proporción.


**Proportion6Price1** **_(decimal)_** : Valor decimal con el PVP1 de la 6ª Proporción.

**Proportion7Price1** **_(decimal)_** : Valor decimal con el PVP1 de la 7ª Proporción.

**Proportion8Price1** **_(decimal)_** : Valor decimal con el PVP1 de la 8ª Proporción.

**Proportion9Price1** **_(decimal)_** : Valor decimal con el PVP1 de la 9ª Proporción.

**Proportion2Price2** **_(decimal)_** : Valor decimal con el PVP2 de la 2ª Proporción.

**Proportion3Price2** **_(decimal)_** : Valor decimal con el PVP2 de la 3ª Proporción.

**Proportion4Price2** **_(decimal)_** : Valor decimal con el PVP2 de la 4ª Proporción.

**Proportion5Price2** **_(decimal)_** : Valor decimal con el PVP2 de la 5ª Proporción.

**Proportion6Price2** **_(decimal)_** : Valor decimal con el PVP2 de la 6ª Proporción.

**Proportion7Price2** **_(decimal)_** : Valor decimal con el PVP2 de la 7ª Proporción.

**Proportion8Price2** **_(decimal)_** : Valor decimal con el PVP2 de la 8ª Proporción.

**Proportion9Price2** **_(decimal)_** : Valor decimal con el PVP2 de la 9ª Proporción.

**Proportion2Price3** **_(decimal)_** : Valor decimal con el PVP3 de la 2ª Proporción.

**Proportion3Price3** **_(decimal)_** : Valor decimal con el PVP3 de la 3ª Proporción.

**Proportion4Price3** **_(decimal)_** : Valor decimal con el PVP3 de la 4ª Proporción.

**Proportion5Price3** **_(decimal)_** : Valor decimal con el PVP3 de la 5ª Proporción.

**Proportion6Price3** **_(decimal)_** : Valor decimal con el PVP3 de la 6ª Proporción.

**Proportion7Price3** **_(decimal)_** : Valor decimal con el PVP3 de la 7ª Proporción.

**Proportion8Price3** **_(decimal)_** : Valor decimal con el PVP3 de la 8ª Proporción.

**Proportion9Price3** **_(decimal)_** : Valor decimal con el PVP3 de la 9ª Proporción.

**Proportion2Price4** **_(decimal)_** : Valor decimal con el PVP4 de la 2ª Proporción.

**Proportion3Price4** **_(decimal)_** : Valor decimal con el PVP4 de la 3ª Proporción.

**Proportion4Price4** **_(decimal)_** : Valor decimal con el PVP4 de la 4ª Proporción.

**Proportion5Price4** **_(decimal)_** : Valor decimal con el PVP4 de la 5ª Proporción.

**Proportion6Price4** **_(decimal)_** : Valor decimal con el PVP4 de la 6ª Proporción.

**Proportion7Price4** **_(decimal)_** : Valor decimal con el PVP4 de la 7ª Proporción.

**Proportion8Price4** **_(decimal)_** : Valor decimal con el PVP4 de la 8ª Proporción.

**Proportion9Price4** **_(decimal)_** : Valor decimal con el PVP4 de la 9ª Proporción.

**Proportion2Price5** **_(decimal)_** : Valor decimal con el PVP5 de la 2ª Proporción.


**Proportion3Price5** **_(decimal)_** : Valor decimal con el PVP5 de la 3ª Proporción.

**Proportion4Price5** **_(decimal)_** : Valor decimal con el PVP5 de la 4ª Proporción.

**Proportion5Price5** **_(decimal)_** : Valor decimal con el PVP5 de la 5ª Proporción.

**Proportion6Price5** **_(decimal)_** : Valor decimal con el PVP5 de la 6ª Proporción.

**Proportion7Price5** **_(decimal)_** : Valor decimal con el PVP5 de la 7ª Proporción.

**Proportion8Price5** **_(decimal)_** : Valor decimal con el PVP5 de la 8ª Proporción.

**Proportion9Price5** **_(decimal)_** : Valor decimal con el PVP5 de la 9ª Proporción.

**Proportion2PluDiscount** **_(integer)_** : Valor entero que corresponde con el campo ‘PLU Descontar de la 2ª Proporción’.

**Proportion2PluDiscountDescription** **_(string)_** : Cadena de texto que corresponde al campo de la descripción del ‘PLU
Descontar de la 2ª Proporción’.

**Proportion3PluDiscount** **_(integer)_** : Valor entero que corresponde con el campo ‘PLU Descontar de la 3ª Proporción’.

**Proportion3PluDiscountDescription** **_(string)_** : Cadena de texto que corresponde al campo de la descripción del ‘PLU
Descontar de la 3ª Proporción’.

**Proportion4PluDiscount** **_(integer)_** : Valor entero que corresponde con el campo ‘PLU Descontar de la 4ª Proporción’.

**Proportion4PluDiscountDescription** **_(string)_** : Cadena de texto que corresponde al campo de la descripción del ‘PLU
Descontar de la 4ª Proporción’.

**Proportion5PluDiscount** **_(integer)_** : Valor entero que corresponde con el campo ‘PLU Descontar de la 5ª Proporción’.

**Proportion5PluDiscountDescription** **_(string)_** : Cadena de texto que corresponde al campo de la descripción del ‘PLU
Descontar de la 5ª Proporción’.

**Proportion6PluDiscount** **_(integer)_** : Valor entero que corresponde con el campo ‘PLU Descontar de la 6ª Proporción’.

**Proportion6PluDiscountDescription** **_(string)_** : Cadena de texto que corresponde al campo de la descripción del ‘PLU
Descontar de la 6ª Proporción’.

**Proportion7PluDiscount** **_(integer)_** : Valor entero que corresponde con el campo ‘PLU Descontar de la 7ª Proporción’.

**Proportion7PluDiscountDescription** **_(string)_** : Cadena de texto que corresponde al campo de la descripción del ‘PLU
Descontar de la 7ª Proporción’.

**Proportion8PluDiscount** **_(integer)_** : Valor entero que corresponde con el campo ‘PLU Descontar de la 8ª Proporción’.

**Proportion8PluDiscountDescription** **_(string)_** : Cadena de texto que corresponde al campo de la descripción del ‘PLU
Descontar de la 8ª Proporción’.

**Proportion9PluDiscount** **_(integer)_** : Valor entero que corresponde con el campo ‘PLU Descontar de la 9ª Proporción’.

**Proportion9PluDiscountDescription** **_(string)_** : Cadena de texto que corresponde al campo de la descripción del ‘PLU
Descontar de la 9ª Proporción’.

**ApplyDiscountsInProp** **_(boolean)_** **:** campo lógico que corresponde al campo ‘Aplicar Descuentos Generales del Artículo en
Proporciones’.


```
ArtCode (long) : Valor numérico de entre 1 y 13 dígitos correspondiente al código de del artículo.
```
##### JSON de respuesta

###### {

```
"NumberItems": 259,
"ErrorMessage": "",
"ArticlesListData": [
{
"DeptCode": 15,
"DeptDescription": "TABACO",
"MenuDish": false,
"WebArticle": false,
"POS_SupplementsProfileID": 0.0,
"SelfOrdering_CommentsProfileID": 0.0,
"SelfOrdering_SupplementsProfileID": 0.0,
"POS_MenuID": 0.0,
"POS_FastfoodID": 0.0,
"POS_PackID": 0.0,
"Is_Inventoriable": true,
"BuyTAVCode": 0,
"BuyTAVPer": 0.0,
"TAVCode": 1,
"TAVPer": 10.0,
"AuxPrinters": "",
"Commissionable": false,
"ModifiablePrice": false,
"DontPrintTicketValue0": false,
"Weight": false,
"DontNotifyUnitsPrice0": true,
"NotifyModifyPriceUnits": true,
"TwoForOne": false,
"POS_CommentsProfileID": 0.0,
"ErrorMessage": null,
"PriceConfirmation": false,
"FreeDescription": false,
"IsCombinable": false,
"CombinedDescription": "",
"CombBasePrice1": 0.0,
"CombBasePrice2": 0.0,
"CombBasePrice3": 0.0,
"CombBasePrice4": 0.0,
"CombBasePrice5": 0.0,
"CombAuxPrice1": 0.0,
"CombAuxPrice2": 0.0,
"CombAuxPrice3": 0.0,
"CombAuxPrice4": 0.0,
"CombAuxPrice5": 0.0,
"ActivateAlwaysCombined": false,
```

"MandatoryCombined": 3,
"CombinedAssocType": 1,
"CombinedDepartmentAssoc": 0,
"CombinedDepartmentAssocDescription": "",
"CombinedMaxiscreenAssoc": 0,
"CombinedMaxiscreenAssocDescription": "",
"ApplyDiscountsInComb": false,
"ArtDescription": "HABANOS PUROS CJ 10 UNI",
"Price1": 5.0,
"Price2": 5.0,
"Price3": 5.0,
"Price4": 5.0,
"Price5": 5.0,
"Dct1": 0.0,
"Dct2": 0.0,
"Dct3": 0.0,
"Dct4": 0.0,
"Dct5": 0.0,
"GraphDescrip1": "PUROS",
"GraphDescrip2": "HABANOS",
"GraphDescrip3": "",
"ExtendedArtDescription": "",
"Proportion1Description": "",
"Proportion2Active": false,
"Proportion3Active": false,
"Proportion4Active": false,
"Proportion5Active": false,
"Proportion6Active": false,
"Proportion7Active": false,
"Proportion8Active": false,
"Proportion9Active": false,
"Proportion2Amount": 0.0,
"Proportion3Amount": 0.0,
"Proportion4Amount": 0.0,
"Proportion5Amount": 0.0,
"Proportion6Amount": 0.0,
"Proportion7Amount": 0.0,
"Proportion8Amount": 0.0,
"Proportion9Amount": 0.0,
"Proportion2Description": "",
"Proportion3Description": "",
"Proportion4Description": "",
"Proportion5Description": "",
"Proportion6Description": "",
"Proportion7Description": "",
"Proportion8Description": "",
"Proportion9Description": "",
"Proportion2Price1": 0.0,
"Proportion3Price1": 0.0,
"Proportion4Price1": 0.0,


"Proportion5Price1": 0.0,
"Proportion6Price1": 0.0,
"Proportion7Price1": 0.0,
"Proportion8Price1": 0.0,
"Proportion9Price1": 0.0,
"Proportion2Price2": 0.0,
"Proportion3Price2": 0.0,
"Proportion4Price2": 0.0,
"Proportion5Price2": 0.0,
"Proportion6Price2": 0.0,
"Proportion7Price2": 0.0,
"Proportion8Price2": 0.0,
"Proportion9Price2": 0.0,
"Proportion2Price3": 0.0,
"Proportion3Price3": 0.0,
"Proportion4Price3": 0.0,
"Proportion5Price3": 0.0,
"Proportion6Price3": 0.0,
"Proportion7Price3": 0.0,
"Proportion8Price3": 0.0,
"Proportion9Price3": 0.0,
"Proportion2Price4": 0.0,
"Proportion3Price4": 0.0,
"Proportion4Price4": 0.0,
"Proportion5Price4": 0.0,
"Proportion6Price4": 0.0,
"Proportion7Price4": 0.0,
"Proportion8Price4": 0.0,
"Proportion9Price4": 0.0,
"Proportion2Price5": 0.0,
"Proportion3Price5": 0.0,
"Proportion4Price5": 0.0,
"Proportion5Price5": 0.0,
"Proportion6Price5": 0.0,
"Proportion7Price5": 0.0,
"Proportion8Price5": 0.0,
"Proportion9Price5": 0.0,
"Proportion2PluDiscount": 0,
"Proportion2PluDiscountDescription": null,
"Proportion3PluDiscount": 0,
"Proportion3PluDiscountDescription": null,
"Proportion4PluDiscount": 0,
"Proportion4PluDiscountDescription": null,
"Proportion5PluDiscount": 0,
"Proportion5PluDiscountDescription": null,
"Proportion6PluDiscount": 0,
"Proportion6PluDiscountDescription": null,
"Proportion7PluDiscount": 0,
"Proportion7PluDiscountDescription": null,
"Proportion8PluDiscount": 0,


"Proportion8PluDiscountDescription": null,
"Proportion9PluDiscount": 0,
"Proportion9PluDiscountDescription": null,
"ApplyDiscountsInProp": false,
"ArtCode": 601
},
{
"DeptCode": 1,
"DeptDescription": "REFRESCOS",
"MenuDish": true,
"WebArticle": true,
"POS_SupplementsProfileID": 0.0,
"SelfOrdering_CommentsProfileID": 0.0,
"SelfOrdering_SupplementsProfileID": 0.0,
"POS_MenuID": 0.0,
"POS_FastfoodID": 0.0,
"POS_PackID": 0.0,
"Is_Inventoriable": true,
"BuyTAVCode": 0,
"BuyTAVPer": 0.0,
"TAVCode": 1,
"TAVPer": 10.0,
"AuxPrinters": "",
"Commissionable": false,
"ModifiablePrice": true,
"DontPrintTicketValue0": false,
"Weight": false,
"DontNotifyUnitsPrice0": false,
"NotifyModifyPriceUnits": false,
"TwoForOne": false,
"POS_CommentsProfileID": 0.0,
"ErrorMessage": null,
"PriceConfirmation": false,
"FreeDescription": false,
"IsCombinable": true,
"CombinedDescription": "COCACOLA",
"CombBasePrice1": 0.0,
"CombBasePrice2": 0.0,
"CombBasePrice3": 0.0,
"CombBasePrice4": 0.0,
"CombBasePrice5": 0.0,
"CombAuxPrice1": 0.6,
"CombAuxPrice2": 0.6,
"CombAuxPrice3": 0.6,
"CombAuxPrice4": 0.6,
"CombAuxPrice5": 0.6,
"ActivateAlwaysCombined": false,
"MandatoryCombined": 2,
"CombinedAssocType": 1,
"CombinedDepartmentAssoc": 0,


"CombinedDepartmentAssocDescription": "",
"CombinedMaxiscreenAssoc": 0,
"CombinedMaxiscreenAssocDescription": "",
"ApplyDiscountsInComb": false,
"ArtDescription": "COCA-COLA",
"Price1": 1.05,
"Price2": 1.05,
"Price3": 1.05,
"Price4": 1.05,
"Price5": 1.05,
"Dct1": 0.0,
"Dct2": 0.0,
"Dct3": 0.0,
"Dct4": 0.0,
"Dct5": 0.0,
"GraphDescrip1": "",
"GraphDescrip2": "",
"GraphDescrip3": "",
"ExtendedArtDescription": "",
"Proportion1Description": "",
"Proportion2Active": false,
"Proportion3Active": false,
"Proportion4Active": false,
"Proportion5Active": false,
"Proportion6Active": false,
"Proportion7Active": false,
"Proportion8Active": false,
"Proportion9Active": false,
"Proportion2Amount": 0.0,
"Proportion3Amount": 0.0,
"Proportion4Amount": 0.0,
"Proportion5Amount": 0.0,
"Proportion6Amount": 0.0,
"Proportion7Amount": 0.0,
"Proportion8Amount": 0.0,
"Proportion9Amount": 0.0,
"Proportion2Description": "",
"Proportion3Description": "",
"Proportion4Description": "",
"Proportion5Description": "",
"Proportion6Description": "",
"Proportion7Description": "",
"Proportion8Description": "",
"Proportion9Description": "",
"Proportion2Price1": 0.0,
"Proportion3Price1": 0.0,
"Proportion4Price1": 0.0,
"Proportion5Price1": 0.0,
"Proportion6Price1": 0.0,
"Proportion7Price1": 0.0,


"Proportion8Price1": 0.0,
"Proportion9Price1": 0.0,
"Proportion2Price2": 0.0,
"Proportion3Price2": 0.0,
"Proportion4Price2": 0.0,
"Proportion5Price2": 0.0,
"Proportion6Price2": 0.0,
"Proportion7Price2": 0.0,
"Proportion8Price2": 0.0,
"Proportion9Price2": 0.0,
"Proportion2Price3": 0.0,
"Proportion3Price3": 0.0,
"Proportion4Price3": 0.0,
"Proportion5Price3": 0.0,
"Proportion6Price3": 0.0,
"Proportion7Price3": 0.0,
"Proportion8Price3": 0.0,
"Proportion9Price3": 0.0,
"Proportion2Price4": 0.0,
"Proportion3Price4": 0.0,
"Proportion4Price4": 0.0,
"Proportion5Price4": 0.0,
"Proportion6Price4": 0.0,
"Proportion7Price4": 0.0,
"Proportion8Price4": 0.0,
"Proportion9Price4": 0.0,
"Proportion2Price5": 0.0,
"Proportion3Price5": 0.0,
"Proportion4Price5": 0.0,
"Proportion5Price5": 0.0,
"Proportion6Price5": 0.0,
"Proportion7Price5": 0.0,
"Proportion8Price5": 0.0,
"Proportion9Price5": 0.0,
"Proportion2PluDiscount": 0,
"Proportion2PluDiscountDescription": null,
"Proportion3PluDiscount": 0,
"Proportion3PluDiscountDescription": null,
"Proportion4PluDiscount": 0,
"Proportion4PluDiscountDescription": null,
"Proportion5PluDiscount": 0,
"Proportion5PluDiscountDescription": null,
"Proportion6PluDiscount": 0,
"Proportion6PluDiscountDescription": null,
"Proportion7PluDiscount": 0,
"Proportion7PluDiscountDescription": null,
"Proportion8PluDiscount": 0,
"Proportion8PluDiscountDescription": null,
"Proportion9PluDiscount": 0,
"Proportion9PluDiscountDescription": null,


"ApplyDiscountsInProp": false,
"ArtCode": 1001
},
{
"DeptCode": 1,
"DeptDescription": "REFRESCOS",
"MenuDish": true,
"WebArticle": true,
"POS_SupplementsProfileID": 0.0,
"SelfOrdering_CommentsProfileID": 0.0,
"SelfOrdering_SupplementsProfileID": 0.0,
"POS_MenuID": 0.0,
"POS_FastfoodID": 0.0,
"POS_PackID": 0.0,
"Is_Inventoriable": true,
"BuyTAVCode": 0,
"BuyTAVPer": 0.0,
"TAVCode": 3,
"TAVPer": 4.0,
"AuxPrinters": "",
"Commissionable": false,
"ModifiablePrice": true,
"DontPrintTicketValue0": false,
"Weight": false,
"DontNotifyUnitsPrice0": false,
"NotifyModifyPriceUnits": false,
"TwoForOne": false,
"POS_CommentsProfileID": 0.0,
"ErrorMessage": null,
"PriceConfirmation": false,
"FreeDescription": false,
"IsCombinable": true,
"CombinedDescription": "S.LIMON",
"CombBasePrice1": 0.0,
"CombBasePrice2": 0.0,
"CombBasePrice3": 0.0,
"CombBasePrice4": 0.0,
"CombBasePrice5": 0.0,
"CombAuxPrice1": 0.6,
"CombAuxPrice2": 0.6,
"CombAuxPrice3": 0.6,
"CombAuxPrice4": 0.6,
"CombAuxPrice5": 0.6,
"ActivateAlwaysCombined": false,
"MandatoryCombined": 2,
"CombinedAssocType": 1,
"CombinedDepartmentAssoc": 0,
"CombinedDepartmentAssocDescription": "",
"CombinedMaxiscreenAssoc": 0,
"CombinedMaxiscreenAssocDescription": "",


"ApplyDiscountsInComb": false,
"ArtDescription": "SCHWEPPES LIMÓN",
"Price1": 1.05,
"Price2": 1.05,
"Price3": 1.05,
"Price4": 1.05,
"Price5": 1.05,
"Dct1": 0.0,
"Dct2": 0.0,
"Dct3": 0.0,
"Dct4": 0.0,
"Dct5": 0.0,
"GraphDescrip1": "",
"GraphDescrip2": "",
"GraphDescrip3": "",
"ExtendedArtDescription": "",
"Proportion1Description": "",
"Proportion2Active": false,
"Proportion3Active": false,
"Proportion4Active": false,
"Proportion5Active": false,
"Proportion6Active": false,
"Proportion7Active": false,
"Proportion8Active": false,
"Proportion9Active": false,
"Proportion2Amount": 0.0,
"Proportion3Amount": 0.0,
"Proportion4Amount": 0.0,
"Proportion5Amount": 0.0,
"Proportion6Amount": 0.0,
"Proportion7Amount": 0.0,
"Proportion8Amount": 0.0,
"Proportion9Amount": 0.0,
"Proportion2Description": "",
"Proportion3Description": "",
"Proportion4Description": "",
"Proportion5Description": "",
"Proportion6Description": "",
"Proportion7Description": "",
"Proportion8Description": "",
"Proportion9Description": "",
"Proportion2Price1": 0.0,
"Proportion3Price1": 0.0,
"Proportion4Price1": 0.0,
"Proportion5Price1": 0.0,
"Proportion6Price1": 0.0,
"Proportion7Price1": 0.0,
"Proportion8Price1": 0.0,
"Proportion9Price1": 0.0,
"Proportion2Price2": 0.0,


"Proportion3Price2": 0.0,
"Proportion4Price2": 0.0,
"Proportion5Price2": 0.0,
"Proportion6Price2": 0.0,
"Proportion7Price2": 0.0,
"Proportion8Price2": 0.0,
"Proportion9Price2": 0.0,
"Proportion2Price3": 0.0,
"Proportion3Price3": 0.0,
"Proportion4Price3": 0.0,
"Proportion5Price3": 0.0,
"Proportion6Price3": 0.0,
"Proportion7Price3": 0.0,
"Proportion8Price3": 0.0,
"Proportion9Price3": 0.0,
"Proportion2Price4": 0.0,
"Proportion3Price4": 0.0,
"Proportion4Price4": 0.0,
"Proportion5Price4": 0.0,
"Proportion6Price4": 0.0,
"Proportion7Price4": 0.0,
"Proportion8Price4": 0.0,
"Proportion9Price4": 0.0,
"Proportion2Price5": 0.0,
"Proportion3Price5": 0.0,
"Proportion4Price5": 0.0,
"Proportion5Price5": 0.0,
"Proportion6Price5": 0.0,
"Proportion7Price5": 0.0,
"Proportion8Price5": 0.0,
"Proportion9Price5": 0.0,
"Proportion2PluDiscount": 0,
"Proportion2PluDiscountDescription": null,
"Proportion3PluDiscount": 0,
"Proportion3PluDiscountDescription": null,
"Proportion4PluDiscount": 0,
"Proportion4PluDiscountDescription": null,
"Proportion5PluDiscount": 0,
"Proportion5PluDiscountDescription": null,
"Proportion6PluDiscount": 0,
"Proportion6PluDiscountDescription": null,
"Proportion7PluDiscount": 0,
"Proportion7PluDiscountDescription": null,
"Proportion8PluDiscount": 0,
"Proportion8PluDiscountDescription": null,
"Proportion9PluDiscount": 0,
"Proportion9PluDiscountDescription": null,
"ApplyDiscountsInProp": false,
"ArtCode": 1002
},


###### {

```
"DeptCode": 1,
"DeptDescription": "REFRESCOS",
"MenuDish": true,
"WebArticle": false,
"POS_SupplementsProfileID": 0.0,
"SelfOrdering_CommentsProfileID": 0.0,
"SelfOrdering_SupplementsProfileID": 0.0,
"POS_MenuID": 0.0,
"POS_FastfoodID": 0.0,
"POS_PackID": 0.0,
"Is_Inventoriable": true,
"BuyTAVCode": 0,
"BuyTAVPer": 0.0,
"TAVCode": 1,
"TAVPer": 10.0,
"AuxPrinters": "",
"Commissionable": false,
"ModifiablePrice": true,
"DontPrintTicketValue0": false,
"Weight": false,
"DontNotifyUnitsPrice0": false,
"NotifyModifyPriceUnits": false,
"TwoForOne": false,
"POS_CommentsProfileID": 0.0,
"ErrorMessage": null,
"PriceConfirmation": false,
"FreeDescription": false,
"IsCombinable": true,
"CombinedDescription": "",
"CombBasePrice1": 0.0,
"CombBasePrice2": 0.0,
"CombBasePrice3": 0.0,
"CombBasePrice4": 0.0,
"CombBasePrice5": 0.0,
"CombAuxPrice1": 0.6,
"CombAuxPrice2": 0.6,
"CombAuxPrice3": 0.6,
"CombAuxPrice4": 0.6,
"CombAuxPrice5": 0.6,
"ActivateAlwaysCombined": false,
"MandatoryCombined": 2,
"CombinedAssocType": 1,
"CombinedDepartmentAssoc": 0,
"CombinedDepartmentAssocDescription": "",
"CombinedMaxiscreenAssoc": 0,
"CombinedMaxiscreenAssocDescription": "",
"ApplyDiscountsInComb": false,
"ArtDescription": "SCHWEPPES NARANJA",
"Price1": 1.05,
```

"Price2": 1.05,
"Price3": 1.05,
"Price4": 1.05,
"Price5": 1.05,
"Dct1": 0.0,
"Dct2": 0.0,
"Dct3": 0.0,
"Dct4": 0.0,
"Dct5": 0.0,
"GraphDescrip1": "",
"GraphDescrip2": "",
"GraphDescrip3": "",
"ExtendedArtDescription": "",
"Proportion1Description": "",
"Proportion2Active": false,
"Proportion3Active": false,
"Proportion4Active": false,
"Proportion5Active": false,
"Proportion6Active": false,
"Proportion7Active": false,
"Proportion8Active": false,
"Proportion9Active": false,
"Proportion2Amount": 0.0,
"Proportion3Amount": 0.0,
"Proportion4Amount": 0.0,
"Proportion5Amount": 0.0,
"Proportion6Amount": 0.0,
"Proportion7Amount": 0.0,
"Proportion8Amount": 0.0,
"Proportion9Amount": 0.0,
"Proportion2Description": "",
"Proportion3Description": "",
"Proportion4Description": "",
"Proportion5Description": "",
"Proportion6Description": "",
"Proportion7Description": "",
"Proportion8Description": "",
"Proportion9Description": "",
"Proportion2Price1": 0.0,
"Proportion3Price1": 0.0,
"Proportion4Price1": 0.0,
"Proportion5Price1": 0.0,
"Proportion6Price1": 0.0,
"Proportion7Price1": 0.0,
"Proportion8Price1": 0.0,
"Proportion9Price1": 0.0,
"Proportion2Price2": 0.0,
"Proportion3Price2": 0.0,
"Proportion4Price2": 0.0,
"Proportion5Price2": 0.0,


"Proportion6Price2": 0.0,
"Proportion7Price2": 0.0,
"Proportion8Price2": 0.0,
"Proportion9Price2": 0.0,
"Proportion2Price3": 0.0,
"Proportion3Price3": 0.0,
"Proportion4Price3": 0.0,
"Proportion5Price3": 0.0,
"Proportion6Price3": 0.0,
"Proportion7Price3": 0.0,
"Proportion8Price3": 0.0,
"Proportion9Price3": 0.0,
"Proportion2Price4": 0.0,
"Proportion3Price4": 0.0,
"Proportion4Price4": 0.0,
"Proportion5Price4": 0.0,
"Proportion6Price4": 0.0,
"Proportion7Price4": 0.0,
"Proportion8Price4": 0.0,
"Proportion9Price4": 0.0,
"Proportion2Price5": 0.0,
"Proportion3Price5": 0.0,
"Proportion4Price5": 0.0,
"Proportion5Price5": 0.0,
"Proportion6Price5": 0.0,
"Proportion7Price5": 0.0,
"Proportion8Price5": 0.0,
"Proportion9Price5": 0.0,
"Proportion2PluDiscount": 0,
"Proportion2PluDiscountDescription": null,
"Proportion3PluDiscount": 0,
"Proportion3PluDiscountDescription": null,
"Proportion4PluDiscount": 0,
"Proportion4PluDiscountDescription": null,
"Proportion5PluDiscount": 0,
"Proportion5PluDiscountDescription": null,
"Proportion6PluDiscount": 0,
"Proportion6PluDiscountDescription": null,
"Proportion7PluDiscount": 0,
"Proportion7PluDiscountDescription": null,
"Proportion8PluDiscount": 0,
"Proportion8PluDiscountDescription": null,
"Proportion9PluDiscount": 0,
"Proportion9PluDiscountDescription": null,
"ApplyDiscountsInProp": false,
"ArtCode": 1003
},
{
"DeptCode": 1,
"DeptDescription": "REFRESCOS",


"MenuDish": false,
"WebArticle": false,
"POS_SupplementsProfileID": 0.0,
"SelfOrdering_CommentsProfileID": 0.0,
"SelfOrdering_SupplementsProfileID": 0.0,
"POS_MenuID": 0.0,
"POS_FastfoodID": 0.0,
"POS_PackID": 0.0,
"Is_Inventoriable": true,
"BuyTAVCode": 0,
"BuyTAVPer": 0.0,
"TAVCode": 1,
"TAVPer": 10.0,
"AuxPrinters": "",
"Commissionable": false,
"ModifiablePrice": true,
"DontPrintTicketValue0": false,
"Weight": false,
"DontNotifyUnitsPrice0": false,
"NotifyModifyPriceUnits": false,
"TwoForOne": false,
"POS_CommentsProfileID": 0.0,
"ErrorMessage": null,
"PriceConfirmation": false,
"FreeDescription": false,
"IsCombinable": false,
"CombinedDescription": "",
"CombBasePrice1": 1.2,
"CombBasePrice2": 1.2,
"CombBasePrice3": 1.2,
"CombBasePrice4": 1.2,
"CombBasePrice5": 1.2,
"CombAuxPrice1": 0.0,
"CombAuxPrice2": 0.0,
"CombAuxPrice3": 0.0,
"CombAuxPrice4": 0.0,
"CombAuxPrice5": 0.0,
"ActivateAlwaysCombined": false,
"MandatoryCombined": 3,
"CombinedAssocType": 1,
"CombinedDepartmentAssoc": 0,
"CombinedDepartmentAssocDescription": "",
"CombinedMaxiscreenAssoc": 0,
"CombinedMaxiscreenAssocDescription": "",
"ApplyDiscountsInComb": false,
"ArtDescription": "CINZANO BITTER",
"Price1": 1.2,
"Price2": 1.2,
"Price3": 1.2,
"Price4": 1.2,


"Price5": 1.2,
"Dct1": 0.0,
"Dct2": 0.0,
"Dct3": 0.0,
"Dct4": 0.0,
"Dct5": 0.0,
"GraphDescrip1": "",
"GraphDescrip2": "",
"GraphDescrip3": "",
"ExtendedArtDescription": "",
"Proportion1Description": "",
"Proportion2Active": false,
"Proportion3Active": false,
"Proportion4Active": false,
"Proportion5Active": false,
"Proportion6Active": false,
"Proportion7Active": false,
"Proportion8Active": false,
"Proportion9Active": false,
"Proportion2Amount": 0.0,
"Proportion3Amount": 0.0,
"Proportion4Amount": 0.0,
"Proportion5Amount": 0.0,
"Proportion6Amount": 0.0,
"Proportion7Amount": 0.0,
"Proportion8Amount": 0.0,
"Proportion9Amount": 0.0,
"Proportion2Description": "",
"Proportion3Description": "",
"Proportion4Description": "",
"Proportion5Description": "",
"Proportion6Description": "",
"Proportion7Description": "",
"Proportion8Description": "",
"Proportion9Description": "",
"Proportion2Price1": 0.0,
"Proportion3Price1": 0.0,
"Proportion4Price1": 0.0,
"Proportion5Price1": 0.0,
"Proportion6Price1": 0.0,
"Proportion7Price1": 0.0,
"Proportion8Price1": 0.0,
"Proportion9Price1": 0.0,
"Proportion2Price2": 0.0,
"Proportion3Price2": 0.0,
"Proportion4Price2": 0.0,
"Proportion5Price2": 0.0,
"Proportion6Price2": 0.0,
"Proportion7Price2": 0.0,
"Proportion8Price2": 0.0,


"Proportion9Price2": 0.0,
"Proportion2Price3": 0.0,
"Proportion3Price3": 0.0,
"Proportion4Price3": 0.0,
"Proportion5Price3": 0.0,
"Proportion6Price3": 0.0,
"Proportion7Price3": 0.0,
"Proportion8Price3": 0.0,
"Proportion9Price3": 0.0,
"Proportion2Price4": 0.0,
"Proportion3Price4": 0.0,
"Proportion4Price4": 0.0,
"Proportion5Price4": 0.0,
"Proportion6Price4": 0.0,
"Proportion7Price4": 0.0,
"Proportion8Price4": 0.0,
"Proportion9Price4": 0.0,
"Proportion2Price5": 0.0,
"Proportion3Price5": 0.0,
"Proportion4Price5": 0.0,
"Proportion5Price5": 0.0,
"Proportion6Price5": 0.0,
"Proportion7Price5": 0.0,
"Proportion8Price5": 0.0,
"Proportion9Price5": 0.0,
"Proportion2PluDiscount": 0,
"Proportion2PluDiscountDescription": null,
"Proportion3PluDiscount": 0,
"Proportion3PluDiscountDescription": null,
"Proportion4PluDiscount": 0,
"Proportion4PluDiscountDescription": null,
"Proportion5PluDiscount": 0,
"Proportion5PluDiscountDescription": null,
"Proportion6PluDiscount": 0,
"Proportion6PluDiscountDescription": null,
"Proportion7PluDiscount": 0,
"Proportion7PluDiscountDescription": null,
"Proportion8PluDiscount": 0,
"Proportion8PluDiscountDescription": null,
"Proportion9PluDiscount": 0,
"Proportion9PluDiscountDescription": null,
"ApplyDiscountsInProp": false,
"ArtCode": 1004
},
{
"DeptCode": 1,
"DeptDescription": "REFRESCOS",
"MenuDish": false,
"WebArticle": false,
"POS_SupplementsProfileID": 0.0,


"SelfOrdering_CommentsProfileID": 0.0,
"SelfOrdering_SupplementsProfileID": 0.0,
"POS_MenuID": 0.0,
"POS_FastfoodID": 0.0,
"POS_PackID": 0.0,
"Is_Inventoriable": true,
"BuyTAVCode": 0,
"BuyTAVPer": 0.0,
"TAVCode": 1,
"TAVPer": 10.0,
"AuxPrinters": "",
"Commissionable": false,
"ModifiablePrice": true,
"DontPrintTicketValue0": false,
"Weight": false,
"DontNotifyUnitsPrice0": false,
"NotifyModifyPriceUnits": false,
"TwoForOne": false,
"POS_CommentsProfileID": 0.0,
"ErrorMessage": null,
"PriceConfirmation": false,
"FreeDescription": false,
"IsCombinable": true,
"CombinedDescription": "",
"CombBasePrice1": 0.0,
"CombBasePrice2": 0.0,
"CombBasePrice3": 0.0,
"CombBasePrice4": 0.0,
"CombBasePrice5": 0.0,
"CombAuxPrice1": 0.6,
"CombAuxPrice2": 0.6,
"CombAuxPrice3": 0.6,
"CombAuxPrice4": 0.6,
"CombAuxPrice5": 0.6,
"ActivateAlwaysCombined": false,
"MandatoryCombined": 2,
"CombinedAssocType": 1,
"CombinedDepartmentAssoc": 0,
"CombinedDepartmentAssocDescription": "",
"CombinedMaxiscreenAssoc": 0,
"CombinedMaxiscreenAssocDescription": "",
"ApplyDiscountsInComb": false,
"ArtDescription": "PEPSI COLA",
"Price1": 1.05,
"Price2": 1.05,
"Price3": 1.05,
"Price4": 1.05,
"Price5": 1.05,
"Dct1": 0.0,
"Dct2": 0.0,


"Dct3": 0.0,
"Dct4": 0.0,
"Dct5": 0.0,
"GraphDescrip1": "",
"GraphDescrip2": "",
"GraphDescrip3": "",
"ExtendedArtDescription": "",
"Proportion1Description": "",
"Proportion2Active": false,
"Proportion3Active": false,
"Proportion4Active": false,
"Proportion5Active": false,
"Proportion6Active": false,
"Proportion7Active": false,
"Proportion8Active": false,
"Proportion9Active": false,
"Proportion2Amount": 0.0,
"Proportion3Amount": 0.0,
"Proportion4Amount": 0.0,
"Proportion5Amount": 0.0,
"Proportion6Amount": 0.0,
"Proportion7Amount": 0.0,
"Proportion8Amount": 0.0,
"Proportion9Amount": 0.0,
"Proportion2Description": "",
"Proportion3Description": "",
"Proportion4Description": "",
"Proportion5Description": "",
"Proportion6Description": "",
"Proportion7Description": "",
"Proportion8Description": "",
"Proportion9Description": "",
"Proportion2Price1": 0.0,
"Proportion3Price1": 0.0,
"Proportion4Price1": 0.0,
"Proportion5Price1": 0.0,
"Proportion6Price1": 0.0,
"Proportion7Price1": 0.0,
"Proportion8Price1": 0.0,
"Proportion9Price1": 0.0,
"Proportion2Price2": 0.0,
"Proportion3Price2": 0.0,
"Proportion4Price2": 0.0,
"Proportion5Price2": 0.0,
"Proportion6Price2": 0.0,
"Proportion7Price2": 0.0,
"Proportion8Price2": 0.0,
"Proportion9Price2": 0.0,
"Proportion2Price3": 0.0,
"Proportion3Price3": 0.0,


"Proportion4Price3": 0.0,
"Proportion5Price3": 0.0,
"Proportion6Price3": 0.0,
"Proportion7Price3": 0.0,
"Proportion8Price3": 0.0,
"Proportion9Price3": 0.0,
"Proportion2Price4": 0.0,
"Proportion3Price4": 0.0,
"Proportion4Price4": 0.0,
"Proportion5Price4": 0.0,
"Proportion6Price4": 0.0,
"Proportion7Price4": 0.0,
"Proportion8Price4": 0.0,
"Proportion9Price4": 0.0,
"Proportion2Price5": 0.0,
"Proportion3Price5": 0.0,
"Proportion4Price5": 0.0,
"Proportion5Price5": 0.0,
"Proportion6Price5": 0.0,
"Proportion7Price5": 0.0,
"Proportion8Price5": 0.0,
"Proportion9Price5": 0.0,
"Proportion2PluDiscount": 0,
"Proportion2PluDiscountDescription": null,
"Proportion3PluDiscount": 0,
"Proportion3PluDiscountDescription": null,
"Proportion4PluDiscount": 0,
"Proportion4PluDiscountDescription": null,
"Proportion5PluDiscount": 0,
"Proportion5PluDiscountDescription": null,
"Proportion6PluDiscount": 0,
"Proportion6PluDiscountDescription": null,
"Proportion7PluDiscount": 0,
"Proportion7PluDiscountDescription": null,
"Proportion8PluDiscount": 0,
"Proportion8PluDiscountDescription": null,
"Proportion9PluDiscount": 0,
"Proportion9PluDiscountDescription": null,
"ApplyDiscountsInProp": false,
"ArtCode": 1005
},
{
"DeptCode": 1,
"DeptDescription": "REFRESCOS",
"MenuDish": false,
"WebArticle": false,
"POS_SupplementsProfileID": 0.0,
"SelfOrdering_CommentsProfileID": 0.0,
"SelfOrdering_SupplementsProfileID": 0.0,
"POS_MenuID": 0.0,


"POS_FastfoodID": 0.0,
"POS_PackID": 0.0,
"Is_Inventoriable": true,
"BuyTAVCode": 0,
"BuyTAVPer": 0.0,
"TAVCode": 1,
"TAVPer": 10.0,
"AuxPrinters": "",
"Commissionable": false,
"ModifiablePrice": true,
"DontPrintTicketValue0": false,
"Weight": false,
"DontNotifyUnitsPrice0": false,
"NotifyModifyPriceUnits": false,
"TwoForOne": false,
"POS_CommentsProfileID": 0.0,
"ErrorMessage": null,
"PriceConfirmation": false,
"FreeDescription": false,
"IsCombinable": false,
"CombinedDescription": "",
"CombBasePrice1": 0.0,
"CombBasePrice2": 0.0,
"CombBasePrice3": 0.0,
"CombBasePrice4": 0.0,
"CombBasePrice5": 0.0,
"CombAuxPrice1": 0.0,
"CombAuxPrice2": 0.0,
"CombAuxPrice3": 0.0,
"CombAuxPrice4": 0.0,
"CombAuxPrice5": 0.0,
"ActivateAlwaysCombined": false,
"MandatoryCombined": 3,
"CombinedAssocType": 1,
"CombinedDepartmentAssoc": 0,
"CombinedDepartmentAssocDescription": "",
"CombinedMaxiscreenAssoc": 0,
"CombinedMaxiscreenAssocDescription": "",
"ApplyDiscountsInComb": false,
"ArtDescription": "AGUA MINERAL 1/2 L",
"Price1": 0.6,
"Price2": 0.6,
"Price3": 0.6,
"Price4": 0.6,
"Price5": 0.6,
"Dct1": 0.0,
"Dct2": 0.0,
"Dct3": 0.0,
"Dct4": 0.0,
"Dct5": 0.0,


"GraphDescrip1": "",
"GraphDescrip2": "",
"GraphDescrip3": "",
"ExtendedArtDescription": "",
"Proportion1Description": "",
"Proportion2Active": false,
"Proportion3Active": false,
"Proportion4Active": false,
"Proportion5Active": false,
"Proportion6Active": false,
"Proportion7Active": false,
"Proportion8Active": false,
"Proportion9Active": false,
"Proportion2Amount": 0.0,
"Proportion3Amount": 0.0,
"Proportion4Amount": 0.0,
"Proportion5Amount": 0.0,
"Proportion6Amount": 0.0,
"Proportion7Amount": 0.0,
"Proportion8Amount": 0.0,
"Proportion9Amount": 0.0,
"Proportion2Description": "",
"Proportion3Description": "",
"Proportion4Description": "",
"Proportion5Description": "",
"Proportion6Description": "",
"Proportion7Description": "",
"Proportion8Description": "",
"Proportion9Description": "",
"Proportion2Price1": 0.0,
"Proportion3Price1": 0.0,
"Proportion4Price1": 0.0,
"Proportion5Price1": 0.0,
"Proportion6Price1": 0.0,
"Proportion7Price1": 0.0,
"Proportion8Price1": 0.0,
"Proportion9Price1": 0.0,
"Proportion2Price2": 0.0,
"Proportion3Price2": 0.0,
"Proportion4Price2": 0.0,
"Proportion5Price2": 0.0,
"Proportion6Price2": 0.0,
"Proportion7Price2": 0.0,
"Proportion8Price2": 0.0,
"Proportion9Price2": 0.0,
"Proportion2Price3": 0.0,
"Proportion3Price3": 0.0,
"Proportion4Price3": 0.0,
"Proportion5Price3": 0.0,
"Proportion6Price3": 0.0,


"Proportion7Price3": 0.0,
"Proportion8Price3": 0.0,
"Proportion9Price3": 0.0,
"Proportion2Price4": 0.0,
"Proportion3Price4": 0.0,
"Proportion4Price4": 0.0,
"Proportion5Price4": 0.0,
"Proportion6Price4": 0.0,
"Proportion7Price4": 0.0,
"Proportion8Price4": 0.0,
"Proportion9Price4": 0.0,
"Proportion2Price5": 0.0,
"Proportion3Price5": 0.0,
"Proportion4Price5": 0.0,
"Proportion5Price5": 0.0,
"Proportion6Price5": 0.0,
"Proportion7Price5": 0.0,
"Proportion8Price5": 0.0,
"Proportion9Price5": 0.0,
"Proportion2PluDiscount": 0,
"Proportion2PluDiscountDescription": null,
"Proportion3PluDiscount": 0,
"Proportion3PluDiscountDescription": null,
"Proportion4PluDiscount": 0,
"Proportion4PluDiscountDescription": null,
"Proportion5PluDiscount": 0,
"Proportion5PluDiscountDescription": null,
"Proportion6PluDiscount": 0,
"Proportion6PluDiscountDescription": null,
"Proportion7PluDiscount": 0,
"Proportion7PluDiscountDescription": null,
"Proportion8PluDiscount": 0,
"Proportion8PluDiscountDescription": null,
"Proportion9PluDiscount": 0,
"Proportion9PluDiscountDescription": null,
"ApplyDiscountsInProp": false,
"ArtCode": 1006
},
{
"DeptCode": 1,
"DeptDescription": "REFRESCOS",
"MenuDish": false,
"WebArticle": false,
"POS_SupplementsProfileID": 0.0,
"SelfOrdering_CommentsProfileID": 0.0,
"SelfOrdering_SupplementsProfileID": 0.0,
"POS_MenuID": 0.0,
"POS_FastfoodID": 0.0,
"POS_PackID": 0.0,
"Is_Inventoriable": true,


"BuyTAVCode": 0,
"BuyTAVPer": 0.0,
"TAVCode": 1,
"TAVPer": 10.0,
"AuxPrinters": "",
"Commissionable": false,
"ModifiablePrice": true,
"DontPrintTicketValue0": false,
"Weight": false,
"DontNotifyUnitsPrice0": false,
"NotifyModifyPriceUnits": false,
"TwoForOne": false,
"POS_CommentsProfileID": 0.0,
"ErrorMessage": null,
"PriceConfirmation": false,
"FreeDescription": false,
"IsCombinable": true,
"CombinedDescription": "",
"CombBasePrice1": 0.0,
"CombBasePrice2": 0.0,
"CombBasePrice3": 0.0,
"CombBasePrice4": 0.0,
"CombBasePrice5": 0.0,
"CombAuxPrice1": 0.6,
"CombAuxPrice2": 0.6,
"CombAuxPrice3": 0.6,
"CombAuxPrice4": 0.6,
"CombAuxPrice5": 0.6,
"ActivateAlwaysCombined": false,
"MandatoryCombined": 2,
"CombinedAssocType": 1,
"CombinedDepartmentAssoc": 0,
"CombinedDepartmentAssocDescription": "",
"CombinedMaxiscreenAssoc": 0,
"CombinedMaxiscreenAssocDescription": "",
"ApplyDiscountsInComb": false,
"ArtDescription": "ZUMO DE MELOCOTON",
"Price1": 1.2,
"Price2": 1.2,
"Price3": 1.2,
"Price4": 1.2,
"Price5": 1.2,
"Dct1": 0.0,
"Dct2": 0.0,
"Dct3": 0.0,
"Dct4": 0.0,
"Dct5": 0.0,
"GraphDescrip1": "",
"GraphDescrip2": "",
"GraphDescrip3": "",


"ExtendedArtDescription": "",
"Proportion1Description": "",
"Proportion2Active": false,
"Proportion3Active": false,
"Proportion4Active": false,
"Proportion5Active": false,
"Proportion6Active": false,
"Proportion7Active": false,
"Proportion8Active": false,
"Proportion9Active": false,
"Proportion2Amount": 0.0,
"Proportion3Amount": 0.0,
"Proportion4Amount": 0.0,
"Proportion5Amount": 0.0,
"Proportion6Amount": 0.0,
"Proportion7Amount": 0.0,
"Proportion8Amount": 0.0,
"Proportion9Amount": 0.0,
"Proportion2Description": "",
"Proportion3Description": "",
"Proportion4Description": "",
"Proportion5Description": "",
"Proportion6Description": "",
"Proportion7Description": "",
"Proportion8Description": "",
"Proportion9Description": "",
"Proportion2Price1": 0.0,
"Proportion3Price1": 0.0,
"Proportion4Price1": 0.0,
"Proportion5Price1": 0.0,
"Proportion6Price1": 0.0,
"Proportion7Price1": 0.0,
"Proportion8Price1": 0.0,
"Proportion9Price1": 0.0,
"Proportion2Price2": 0.0,
"Proportion3Price2": 0.0,
"Proportion4Price2": 0.0,
"Proportion5Price2": 0.0,
"Proportion6Price2": 0.0,
"Proportion7Price2": 0.0,
"Proportion8Price2": 0.0,
"Proportion9Price2": 0.0,
"Proportion2Price3": 0.0,
"Proportion3Price3": 0.0,
"Proportion4Price3": 0.0,
"Proportion5Price3": 0.0,
"Proportion6Price3": 0.0,
"Proportion7Price3": 0.0,
"Proportion8Price3": 0.0,
"Proportion9Price3": 0.0,


"Proportion2Price4": 0.0,
"Proportion3Price4": 0.0,
"Proportion4Price4": 0.0,
"Proportion5Price4": 0.0,
"Proportion6Price4": 0.0,
"Proportion7Price4": 0.0,
"Proportion8Price4": 0.0,
"Proportion9Price4": 0.0,
"Proportion2Price5": 0.0,
"Proportion3Price5": 0.0,
"Proportion4Price5": 0.0,
"Proportion5Price5": 0.0,
"Proportion6Price5": 0.0,
"Proportion7Price5": 0.0,
"Proportion8Price5": 0.0,
"Proportion9Price5": 0.0,
"Proportion2PluDiscount": 0,
"Proportion2PluDiscountDescription": null,
"Proportion3PluDiscount": 0,
"Proportion3PluDiscountDescription": null,
"Proportion4PluDiscount": 0,
"Proportion4PluDiscountDescription": null,
"Proportion5PluDiscount": 0,
"Proportion5PluDiscountDescription": null,
"Proportion6PluDiscount": 0,
"Proportion6PluDiscountDescription": null,
"Proportion7PluDiscount": 0,
"Proportion7PluDiscountDescription": null,
"Proportion8PluDiscount": 0,
"Proportion8PluDiscountDescription": null,
"Proportion9PluDiscount": 0,
"Proportion9PluDiscountDescription": null,
"ApplyDiscountsInProp": false,
"ArtCode": 1007
},
{
"DeptCode": 1,
"DeptDescription": "REFRESCOS",
"MenuDish": false,
"WebArticle": false,
"POS_SupplementsProfileID": 0.0,
"SelfOrdering_CommentsProfileID": 0.0,
"SelfOrdering_SupplementsProfileID": 0.0,
"POS_MenuID": 0.0,
"POS_FastfoodID": 0.0,
"POS_PackID": 0.0,
"Is_Inventoriable": true,
"BuyTAVCode": 0,
"BuyTAVPer": 0.0,
"TAVCode": 1,


"TAVPer": 10.0,
"AuxPrinters": "",
"Commissionable": false,
"ModifiablePrice": true,
"DontPrintTicketValue0": false,
"Weight": false,
"DontNotifyUnitsPrice0": false,
"NotifyModifyPriceUnits": false,
"TwoForOne": false,
"POS_CommentsProfileID": 0.0,
"ErrorMessage": null,
"PriceConfirmation": false,
"FreeDescription": false,
"IsCombinable": true,
"CombinedDescription": "",
"CombBasePrice1": 0.0,
"CombBasePrice2": 0.0,
"CombBasePrice3": 0.0,
"CombBasePrice4": 0.0,
"CombBasePrice5": 0.0,
"CombAuxPrice1": 0.6,
"CombAuxPrice2": 0.6,
"CombAuxPrice3": 0.6,
"CombAuxPrice4": 0.6,
"CombAuxPrice5": 0.6,
"ActivateAlwaysCombined": false,
"MandatoryCombined": 2,
"CombinedAssocType": 1,
"CombinedDepartmentAssoc": 0,
"CombinedDepartmentAssocDescription": "",
"CombinedMaxiscreenAssoc": 0,
"CombinedMaxiscreenAssocDescription": "",
"ApplyDiscountsInComb": false,
"ArtDescription": "ZUMO DE PIÑA",
"Price1": 1.2,
"Price2": 1.2,
"Price3": 1.2,
"Price4": 1.2,
"Price5": 1.2,
"Dct1": 0.0,
"Dct2": 0.0,
"Dct3": 0.0,
"Dct4": 0.0,
"Dct5": 0.0,
"GraphDescrip1": "",
"GraphDescrip2": "",
"GraphDescrip3": "",
"ExtendedArtDescription": "",
"Proportion1Description": "",
"Proportion2Active": false,


"Proportion3Active": false,
"Proportion4Active": false,
"Proportion5Active": false,
"Proportion6Active": false,
"Proportion7Active": false,
"Proportion8Active": false,
"Proportion9Active": false,
"Proportion2Amount": 0.0,
"Proportion3Amount": 0.0,
"Proportion4Amount": 0.0,
"Proportion5Amount": 0.0,
"Proportion6Amount": 0.0,
"Proportion7Amount": 0.0,
"Proportion8Amount": 0.0,
"Proportion9Amount": 0.0,
"Proportion2Description": "",
"Proportion3Description": "",
"Proportion4Description": "",
"Proportion5Description": "",
"Proportion6Description": "",
"Proportion7Description": "",
"Proportion8Description": "",
"Proportion9Description": "",
"Proportion2Price1": 0.0,
"Proportion3Price1": 0.0,
"Proportion4Price1": 0.0,
"Proportion5Price1": 0.0,
"Proportion6Price1": 0.0,
"Proportion7Price1": 0.0,
"Proportion8Price1": 0.0,
"Proportion9Price1": 0.0,
"Proportion2Price2": 0.0,
"Proportion3Price2": 0.0,
"Proportion4Price2": 0.0,
"Proportion5Price2": 0.0,
"Proportion6Price2": 0.0,
"Proportion7Price2": 0.0,
"Proportion8Price2": 0.0,
"Proportion9Price2": 0.0,
"Proportion2Price3": 0.0,
"Proportion3Price3": 0.0,
"Proportion4Price3": 0.0,
"Proportion5Price3": 0.0,
"Proportion6Price3": 0.0,
"Proportion7Price3": 0.0,
"Proportion8Price3": 0.0,
"Proportion9Price3": 0.0,
"Proportion2Price4": 0.0,
"Proportion3Price4": 0.0,
"Proportion4Price4": 0.0,


"Proportion5Price4": 0.0,
"Proportion6Price4": 0.0,
"Proportion7Price4": 0.0,
"Proportion8Price4": 0.0,
"Proportion9Price4": 0.0,
"Proportion2Price5": 0.0,
"Proportion3Price5": 0.0,
"Proportion4Price5": 0.0,
"Proportion5Price5": 0.0,
"Proportion6Price5": 0.0,
"Proportion7Price5": 0.0,
"Proportion8Price5": 0.0,
"Proportion9Price5": 0.0,
"Proportion2PluDiscount": 0,
"Proportion2PluDiscountDescription": null,
"Proportion3PluDiscount": 0,
"Proportion3PluDiscountDescription": null,
"Proportion4PluDiscount": 0,
"Proportion4PluDiscountDescription": null,
"Proportion5PluDiscount": 0,
"Proportion5PluDiscountDescription": null,
"Proportion6PluDiscount": 0,
"Proportion6PluDiscountDescription": null,
"Proportion7PluDiscount": 0,
"Proportion7PluDiscountDescription": null,
"Proportion8PluDiscount": 0,
"Proportion8PluDiscountDescription": null,
"Proportion9PluDiscount": 0,
"Proportion9PluDiscountDescription": null,
"ApplyDiscountsInProp": false,
"ArtCode": 1008
},
{
"DeptCode": 1,
"DeptDescription": "REFRESCOS",
"MenuDish": false,
"WebArticle": false,
"POS_SupplementsProfileID": 0.0,
"SelfOrdering_CommentsProfileID": 0.0,
"SelfOrdering_SupplementsProfileID": 0.0,
"POS_MenuID": 0.0,
"POS_FastfoodID": 0.0,
"POS_PackID": 0.0,
"Is_Inventoriable": true,
"BuyTAVCode": 0,
"BuyTAVPer": 0.0,
"TAVCode": 1,
"TAVPer": 10.0,
"AuxPrinters": "",
"Commissionable": false,


"ModifiablePrice": true,
"DontPrintTicketValue0": false,
"Weight": false,
"DontNotifyUnitsPrice0": false,
"NotifyModifyPriceUnits": false,
"TwoForOne": false,
"POS_CommentsProfileID": 0.0,
"ErrorMessage": null,
"PriceConfirmation": false,
"FreeDescription": false,
"IsCombinable": true,
"CombinedDescription": "",
"CombBasePrice1": 0.0,
"CombBasePrice2": 0.0,
"CombBasePrice3": 0.0,
"CombBasePrice4": 0.0,
"CombBasePrice5": 0.0,
"CombAuxPrice1": 0.6,
"CombAuxPrice2": 0.6,
"CombAuxPrice3": 0.6,
"CombAuxPrice4": 0.6,
"CombAuxPrice5": 0.6,
"ActivateAlwaysCombined": false,
"MandatoryCombined": 2,
"CombinedAssocType": 1,
"CombinedDepartmentAssoc": 0,
"CombinedDepartmentAssocDescription": "",
"CombinedMaxiscreenAssoc": 0,
"CombinedMaxiscreenAssocDescription": "",
"ApplyDiscountsInComb": false,
"ArtDescription": "ZUMO DE NARANJA",
"Price1": 1.2,
"Price2": 1.2,
"Price3": 1.2,
"Price4": 1.2,
"Price5": 1.2,
"Dct1": 0.0,
"Dct2": 0.0,
"Dct3": 0.0,
"Dct4": 0.0,
"Dct5": 0.0,
"GraphDescrip1": "",
"GraphDescrip2": "",
"GraphDescrip3": "",
"ExtendedArtDescription": "",
"Proportion1Description": "",
"Proportion2Active": false,
"Proportion3Active": false,
"Proportion4Active": false,
"Proportion5Active": false,


"Proportion6Active": false,
"Proportion7Active": false,
"Proportion8Active": false,
"Proportion9Active": false,
"Proportion2Amount": 0.0,
"Proportion3Amount": 0.0,
"Proportion4Amount": 0.0,
"Proportion5Amount": 0.0,
"Proportion6Amount": 0.0,
"Proportion7Amount": 0.0,
"Proportion8Amount": 0.0,
"Proportion9Amount": 0.0,
"Proportion2Description": "",
"Proportion3Description": "",
"Proportion4Description": "",
"Proportion5Description": "",
"Proportion6Description": "",
"Proportion7Description": "",
"Proportion8Description": "",
"Proportion9Description": "",
"Proportion2Price1": 0.0,
"Proportion3Price1": 0.0,
"Proportion4Price1": 0.0,
"Proportion5Price1": 0.0,
"Proportion6Price1": 0.0,
"Proportion7Price1": 0.0,
"Proportion8Price1": 0.0,
"Proportion9Price1": 0.0,
"Proportion2Price2": 0.0,
"Proportion3Price2": 0.0,
"Proportion4Price2": 0.0,
"Proportion5Price2": 0.0,
"Proportion6Price2": 0.0,
"Proportion7Price2": 0.0,
"Proportion8Price2": 0.0,
"Proportion9Price2": 0.0,
"Proportion2Price3": 0.0,
"Proportion3Price3": 0.0,
"Proportion4Price3": 0.0,
"Proportion5Price3": 0.0,
"Proportion6Price3": 0.0,
"Proportion7Price3": 0.0,
"Proportion8Price3": 0.0,
"Proportion9Price3": 0.0,
"Proportion2Price4": 0.0,
"Proportion3Price4": 0.0,
"Proportion4Price4": 0.0,
"Proportion5Price4": 0.0,
"Proportion6Price4": 0.0,
"Proportion7Price4": 0.0,


"Proportion8Price4": 0.0,
"Proportion9Price4": 0.0,
"Proportion2Price5": 0.0,
"Proportion3Price5": 0.0,
"Proportion4Price5": 0.0,
"Proportion5Price5": 0.0,
"Proportion6Price5": 0.0,
"Proportion7Price5": 0.0,
"Proportion8Price5": 0.0,
"Proportion9Price5": 0.0,
"Proportion2PluDiscount": 0,
"Proportion2PluDiscountDescription": null,
"Proportion3PluDiscount": 0,
"Proportion3PluDiscountDescription": null,
"Proportion4PluDiscount": 0,
"Proportion4PluDiscountDescription": null,
"Proportion5PluDiscount": 0,
"Proportion5PluDiscountDescription": null,
"Proportion6PluDiscount": 0,
"Proportion6PluDiscountDescription": null,
"Proportion7PluDiscount": 0,
"Proportion7PluDiscountDescription": null,
"Proportion8PluDiscount": 0,
"Proportion8PluDiscountDescription": null,
"Proportion9PluDiscount": 0,
"Proportion9PluDiscountDescription": null,
"ApplyDiscountsInProp": false,
"ArtCode": 1009
}
]
}


### GetFullArticlesList

Obtiene una lista con la información completa de los artículos de un perfil de departamentos y artículos determinado o de la tabla
principal de artículos. Si se obtienen artículos de un perfil, los campos que no existan en la ficha de los artículos en perfil, se cogerán
de la tabla de artículos. Este comando posee una gran cantidad de parámetros de filtrado.

##### Ruta de llamada

/API/Articles/GetFullList

##### Parámetros de entrada

**Art1** **_(long)_** : Valor entero de 1 a 13 dígitos mayor que cero correspondiente al código inicial del intervalo de artículos que se van a
exportar.

**Art2** **_(long)_** : Valor entero de 1 a 13 dígitos mayor que cero correspondiente al código final del intervalo de artículos que se van a
exportar.

**Dept1** **_(integer)_** : Valor entero de 1 a 3 dígitos mayor que cero correspondiente al código inicial del departamento de venta de los
artículos que se van a exportar.

**Dept2** **_(integer)_** : Valor entero de 1 a 3 dígitos mayor que cero correspondiente al código final del departamento de venta de los
artículos que se van a exportar.

**Description** **_(string)_** : Cadena de texto que permite especificar una cadena de texto. Esta cadena de texto será utilizada para filtrar la
exportación de artículos aplicando el filtro a la descripción del departamento del artículo en cuestión.

**DescriptionQueryType** **_(StringSearchType)_** : Campo te tipo numérico que permitirá especificar el tipo de búsqueda que se realizará
en la descripción del departamento de los artículos para aplicar el filtrado. Los tipos disponibles son:

##### None = 0 : Ningún tipo. No se aplicará filtrado por descripción de departamento.

##### Starts = 1 : que empiece por. Se filtrará la exportación de artículos a artículos que su departamento empiece

##### por la cadena especificada en el campo Description.

##### Contains = 2 : que contenga. Se filtrará la exportación de artículos a artículos que su departamento contenga la

##### cadena especificada en el campo Description.

##### Ends = 3 : que acabe en. Se filtrará la exportación de artículos a artículos que su departamento acabe en la

##### cadena especificada en el campo Description.

**ItemsPerPage(** **_integer_** **)** : Valor numérico que permitirá que los resultados de datos de artículos sean paginados. Mediante este valor
se puede especificar la cantidad de elementos de artículos que contendrá cada página.

**ActualPage(** **_integer_** **)** : Valor numérico que presentará utilidad si se dan los resultados paginados. Permitirá especificar la página
actual.

**nField(** **_GetDepartmentField_** **)** : Campo de tipo numérico que permitirá especificar el campo por el que se deberán ordenar los
resultados. Los tipos de ordenación disponibles son:

```
ArtCode = 1, ArtDescription = 2, Price1 = 3, Price2 = 4, Price3 = 5, Price4 = 6, Price5 = 7, Dct1 = 8, Dct2 = 9, Dct3 = 10, Dct4
= 11, Dct5 = 12, DeptCode = 13, TAVCode = 15, GraphDescrip1 = 17, GraphDescrip2 = 18, GraphDescrip3 = 19.
```
**nOrder(** **_OrderType_** **)** : Campo de tipo numérico que permitirá especificar el tipo de ordenación a aplicar a la ordenación especificada
en el campo anterior que se aplicará a los resultados. Los tipos de ordenación disponibles son:

```
ASC = 0 : ascendente (de menor a mayor).
```
```
DESC = 1 : descendente (de mayor a menor).
```

**ProfileCode:** Valor numérico que permitirá especificar el código de perfil de artículos para el que se van a obtener sus artículos. Si
se especifica 0, los artículos se obtendrán de la tabla principal de artículos.

**IsArticleWeb** **_(int)_** : Campo de tipo numérico que permitirá especificar un filtro de carga de artículos en función de si son artículos
Web o no (parámetro de “Artículo Web” en la pestaña Web del mantenimiento de artículos). Los tipos de filtro disponible son:

**AllArticles = 0** : Se obtendrán todos los artículos independientemente del parámetro “Artículo Web”.

**OnlyWebArticles = 1:** En la consulta se obtendrán artículos que tengan la propiedad “Artículo Web” marcada.

**OnlyNonWebArticles = 2** : En la consulta se obtendrán artículos que no tengan la propiedad “Artículo Web” marcada.

**PriceType** **_(integer)_** : Se podrá filtrar el listado de artículos excluyendo artículos que tengan precio igual a 0. Mediante este valor
entero se podrá especificar la tarifa de precios (1 a 5) que se utilizará para aplicar el filtro de exclusión de artículos con precio 0.

**Store** **_(integer)_** : Se podrá filtrar el listado de artículos excluyendo artículos que tengan stock actual igual a 0. Mediante este valor
entero se podrá especificar el código de almacén del que se obtendrá el stock de los artículos que se utilizará para aplicar el filtro
de exclusión de artículos con stock actual a 0.

**AllowArticlesWithoutStock** **_(boolean)_** : Campo lógico. Si se asigna a ‘True’, en la exportación se incluirán todos los artículos del
intervalo seleccionado, aunque su stock actual sea 0. Si se asigna a ‘False’, de la exportación se excluirán todos los artículos del
intervalo seleccionado que tengan su stock actual a 0.

**AllowArticlesWithPriceZero** **_(boolean)_** : Campo lógico. Si se asigna a ‘True’, en la exportación se incluirán todos los artículos del
intervalo seleccionado, aunque su precio de venta (de la tarifa especificada en el parámetro **PriceType** ) sea 0. Si se asigna a ‘False’,
de la exportación se excluirán todos los artículos del intervalo seleccionado que tengan su precio de venta (de la tarifa especificada
en el parámetro **Price Type** ) a 0.

##### JSON de solicitud

{
"Art1": 0,
"Art2": 9999999999999,
"Dept1": 0,
"Dept2": 999,
"Description": "",
"DescriptionQueryType": 0,
"ItemsPerPage": 10,
"ActualPage": 1,
"nField": 1,
"nOrder": 0,
"ProfileCode": 2,
"IsArticleWeb": 0,
"PriceType": 1,
"Store": 1,
"AllowArticlesWithoutStock": true,
"AllowArticlesWithPriceZero": false
}

##### Parámetros de salida

**NumberItems:** Valor numérico que indicará la cantidad de artículos obtenidos con esta consulta.

**ErrorMessage** **_(string)_** : Cadena que almacenará posibles errores. Si la consulta se realizó correctamente, estará vacía.


**ArticleListData** **_(ArticleListFullDataType)_** : Lista de objetos **_ArticleListFullDataType_** que contendrá los datos completos de los
artículos obtenidos. Cada objeto **_ArticleListFullDataType_** corresponderá a un artículo y estará compuesto por los siguientes
elementos:

```
DeptCode (integer) : Valor con el código de departamento de venta asociado al artículo.
```
```
DeptDescription (string) : Cadena con la descripción del departamento de venta asociado al artículo.
```
```
AuxPrinters (string) : Cadena con la secuencia de impresoras auxiliares asociadas al artículo.
```
```
MenuDish (boolean) : Campo lógico que corresponde con el campo ‘Plato de Menú’ del artículo.
```
```
POS_SupplementsProfileID (decimal) : Valor con el código de perfil de suplementos asociado al artículo.
```
```
SelfOrdering_CommentsProfileID (decimal) : Valor con el código de perfil de comentarios asociado a la configuración de
autocomanda del artículo.
```
```
SelfOrdering_SupplementsProfileID (decimal) : Valor con el código de perfil de suplementos asociado a la configuración de
autocomanda del artículo.
```
```
WebArticle (boolean) : Campo lógico que corresponde con el campo ‘Artículo Web’ del artículo.
```
```
POS_MenuID (decimal) : Valor con el código de definición de menú asociado al artículo.
```
```
POS_FastfoodID (decimal) : Valor con el código de definición fastfood asociado al artículo.
```
```
POS_PackID (decimal) : Valor con el código de definición de pack asociado al artículo.
```
```
Commissionable (boolean) : Campo lógico que corresponde con el campo ‘Artículo Comisionable’ del artículo.
```
```
ModifiablePrice (boolean) : Campo lógico que corresponde con el campo ‘Precio Modificable’ del artículo.
```
```
DontPrintTicketValue0 (boolean) : Campo lógico que corresponde con el campo ‘No Imprimir en Factura si Valor 0’ del
artículo.
```
```
Weight (boolean) : Campo lógico que corresponde con el campo ‘Artículo de Peso’ del artículo.
```
```
DontNotifyUnitsPrice0 (boolean) : Campo lógico que corresponde con el campo ‘No Avisar si unidades o Precio es 0’ del
artículo.
```
```
NotifyModifyPriceUnits (boolean) : Campo lógico que corresponde con el campo ‘Avisar para Modificar Precio y/o
Unidades’ del artículo.
```
```
TwoForOne (boolean) : Campo lógico que corresponde con el campo ‘Artículo Dos x Uno” del artículo.
```
```
POS_CommentsProfileID (decimal) : Valor con el código de perfil de comentarios asociado al artículo.
```
```
PriceConfirmation (boolean) : Campo lógico que corresponde con el campo ‘Confirmación Precio’ del artículo.
```
```
FreeDescription (boolean) : Campo lógico que corresponde con el campo ‘Descripción Libre’ del artículo.
```
#### SAC_Dimensions( DimensionsDataType) : Este parámetro sólo presentará utilidad para la aplicación de talla y color. El

```
objeto DimensionsDataType contendrá todos los datos de cada una de las tres dimensiones de talla y color asociadas al
artículo en cuestión. Estará compuesto por los siguientes elementos:
```
```
Active(bool): Campo lógico que indicará si el artículo tiene activada la dimensión correspondiente a la dimensión
de este objeto.
```

```
Index(decimal): Valor numérico que indica el índice de la dimensión (de la 1 a la 3) correspondiente a la dimensión
de este objeto.
```
```
Name( string ): Cadena de texto que corresponde a la Descripción de la dimensión correspondiente a la dimensión
de este objeto.
```
```
Values( ValuesDimensionDataType ): Lista de objetos ValuesDimensionDataType que contendrá los valores de la
dimensión correspondiente a la dimensión de este objeto. Cada objeto ValuesDimensionDataType corresponderá
a un valor de dimensión y estará compuesto por los siguientes elementos:
```
```
Description( string ): Cadena de texto que corresponde a la descripción del valor perteneciente de la
dimensión.
```
```
LineId( int ): Valor numérico que muestra el Id de línea de este valor de dimensión.
```
```
OrderId( int ): a Valor numérico que muestra el Id de orden de este valor de dimensión.
```
#### SAC_PricesTable( PricesTableDataType) : Este parámetro sólo presentará utilidad para la aplicación de talla y color. Lista de

objetos **PricesTableDataType** que contendrá los precios de venta y datos de stock de todas las combinaciones de
definiciones de los artículos. Cada objeto **PricesTableDataType** corresponderá a los precios de venta y datos de stock de
una combinación de dimensiones de un artículo (para cada artículo se generará un objeto de este tipo por cada combinación
de definiciones que disponga) y estará compuesto por los siguientes elementos:

```
ArticleId (long) : Valor numérico de entre 1 y 13 dígitos correspondiente al código de del artículo.
```
```
D1LineId(int): Valor numérico que muestra el id de línea del valor de la dimensión 1 de este objeto.
```
```
D2LineId(int): Valor numérico que muestra el id de línea del valor de la dimensión 2 de este objeto.
```
```
D3LineId(int): Valor numérico que muestra el id de línea del valor de la dimensión 3 de este objeto.
```
```
AltenativeCodeBDP(int): Valor numérico que identifica de forma única un artículo de talla y color.
```
```
SaleInfo(SaleInfo): Objeto que contendrá información sobre precios y descuentos de esta combinación de
dimensiones del artículo.
```
```
Price (decimal) : Valor decimal con el precio de esta combinación de dimensiones del artículo
correspondiente al nivel de precio que se especificó en la consulta.
```
```
Discount (decimal) : Valor decimal con el porcentaje de descuento de esta combinación de dimensiones
del artículo correspondiente al nivel de precio que se especificó en la consulta.
```
```
PriceWithDiscount (decimal) : Valor decimal con el precio del campo Price aplicándole el descuento del
campo Discount.
```
```
SalePrice (decimal) : Valor decimal que indicará el precio de rebaja general de esta combinación de
dimensiones del artículo correspondiente al nivel de precio de rebaja que aplicaría la aplicación en el
momento de hacer la consulta.
```
```
SaleDiscount (decimal) : Valor decimal que indicará el porcentaje de rebaja general de esta combinación
de dimensiones del artículo correspondiente al nivel de precio de rebaja que aplicaría la aplicación en el
momento de hacer la consulta.
```
```
SalePriceWithDiscount (decimal) : Valor decimal con el precio del campo SalePrice aplicándole el
descuento del campo SaleDiscount.
```

```
CurrentStock (decimal) : Valor numérico que mostrará el stock actual de esta combinación de dimensiones
del artículo en el almacén correspondiente al código de almacén especificado en la consulta.
```
```
OnSale(bool): Campo lógico que indicará si al artículo, en el momento de realizar la consulta, al artículo
se le aplicaría algún descuento o rebaja en la aplicación.
```
```
PriceToApply(decimal): Valor numérico que indica el precio al que se vendería la combinación de
dimensiones en cuestión del artículo en la aplicación en el momento de realizar la consulta si no se le
aplicara ningún descuento o rebaja.
```
```
SalePriceToApply(decimal): Valor numérico que indica el precio al que se vendería el artículo en la
aplicación en el momento de realizar la consulta aplicando los posibles descuentos o rebajas del artículo.
```
**Is_Inventoriable (boolean)** : Valor lógico que especifica que el artículo es o no es inventariable.

**BuyTAVCode** **_(integer)_** : Valor con el código de IVA de compra asociado al artículo.

**BuyTAVPer** **_(decimal)_** : Valor decimal con el porcentaje del IVA de compra asociado al artículo.

**WebArticle** **_(boolean)_** **:** Valor lógico que indica si el artículo es tipo Web.

**TAVCode** **_(integer)_** : Valor con el código de IVA de venta asociado al artículo.

**TAVPer** **_(decimal)_** : Valor decimal con el porcentaje del IVA de venta asociado al artículo.

**ErrorMessage** **_(string)_** : Cadena que almacenará posibles errores. Si la consulta se realizó correctamente, estará vacía.

**ArtDescription** **_(string)_** : Cadena de texto que corresponde a la descripción del artículo (máx. 40 caracteres).

**SaleInfo(SaleInfo):** Objeto que contendrá información sobre precios y descuentos del artículo.

```
Price (decimal) : Valor decimal con el precio correspondiente al nivel de precio que se especificó en la consulta.
```
```
Discount (decimal) : Valor decimal con el porcentaje de descuento correspondiente al nivel de precio que se
especificó en la consulta.
```
```
PriceWithDiscount (decimal) : Valor decimal con el precio del campo Price aplicándole el descuento del campo
Discount.
```
```
SalePrice (decimal) : Valor decimal que indicará el precio de rebaja general (no el de las líneas de precios de talla y
color) correspondiente al nivel de precio de rebaja que aplicaría la aplicación en el momento de hacer la consulta.
Campo sólo válido si la aplicación corresponde a la de talla y color. Si no es una aplicación de talla y color este valor
será 0.
```
```
SaleDiscount (decimal) : Valor decimal que indicará el porcentaje de rebaja general (no el de las líneas de precios
de talla y color) correspondiente al nivel de precio de rebaja que aplicaría la aplicación en el momento de hacer la
consulta. Campo sólo válido si la aplicación corresponde a la de talla y color. Si no es una aplicación de talla y color
este valor será 0.
```
```
SalePriceWithDiscount (decimal) : Valor decimal con el precio del campo SalePrice aplicándole el descuento del
campo SaleDiscount.
```
```
CurrentStock (decimal) : Valor numérico que mostrará el stock actual del artículo en el almacén correspondiente al
código de almacén especificado en la consulta.
```
```
OnSale(bool): Campo lógico que indicará si al artículo, en el momento de realizar la consulta, al artículo se le
aplicaría algún descuento o rebaja en la aplicación.
```

```
PriceToApply(decimal): Valor numérico que indica el precio al que se vendería el artículo en la aplicación en el
momento de realizar la consulta si no se le aplicara ningún descuento o rebaja.
```
```
SalePriceToApply(decimal): Valor numérico que indica el precio al que se vendería el artículo en la aplicación en
el momento de realizar la consulta aplicando los posibles descuentos o rebajas del artículo.
```
**GraphDescrip1** **_(string)_** : Cadena de texto que corresponde a la Descripción Gráfica de la Línea 1 del artículo.

**GraphDescrip2** **_(string)_** : Cadena de texto que corresponde a la Descripción Gráfica de la Línea 2 del artículo.

**GraphDescrip3** **_(string)_** : Cadena de texto que corresponde a la Descripción Gráfica de la Línea 3 del artículo.

**ExtendedArtDescription** **_(string)_** : Cadena de texto que corresponde a la Descripción Extendida del artículo.

**ArtCode** **_(long)_** : Valor numérico de entre 1 y 13 dígitos correspondiente al código de del artículo.

#### Alergenos( AlergenosDataType ) : Lista de objetos AlergenosDataType que contendrá los datos de los alérgenos asociados

al artículo en cuestión. Cada objeto **AlergenosDataType** corresponderá a un alérgeno y estará compuesto por los siguientes
elementos:

```
Codigo: Valor numérico correspondiente al código del alérgeno asociado al artículo.
```
```
Descripcion: Cadena con la descripción del alérgeno asociado al artículo.
```
```
Proportion1Description : Descripción de la proporción principal (La proporción 1 corresponde al
artículo rpincipal)
Proportion2Active : Estado de activación de la proporción 2,
Proportion3Active : Estado de activación de la proporción 3,
Proportion4Active : Estado de activación de la proporción 4,
Proportion5Active : Estado de activación de la proporción 5,
Proportion6Active : Estado de activación de la proporción 6,
Proportion7Active : Estado de activación de la proporción 7,
Proportion8Active : Estado de activación de la proporción 8,
Proportion9Active : Estado de activación de la proporción 9,
Proportion2Amount : Multiplicador relativo a la proporción principal para la proporción 2,
Proportion3Amount : Multiplicador relativo a la proporción principal para la proporción 3,
Proportion4Amount : Multiplicador relativo a la proporción principal para la proporción 4,
Proportion5Amount : Multiplicador relativo a la proporción principal para la proporción 5,
Proportion6Amount : Multiplicador relativo a la proporción principal para la proporción 6,
Proportion7Amount : Multiplicador relativo a la proporción principal para la proporción 7,
Proportion8Amount : Multiplicador relativo a la proporción principal para la proporción 8,
Proportion9Amount : Multiplicador relativo a la proporción principal para la proporción 9,
Proportion2Description: Descripción de la proporción 2
Proportion3Description: Descripción de la proporción 3
Proportion4Description: Descripción de la proporción 4
Proportion5Description: Descripción de la proporción 5
Proportion6Description: Descripción de la proporción 6
Proportion7Description: Descripción de la proporción 7
Proportion8Description: Descripción de la proporción 8
Proportion9Description: Descripción de la proporción 9
Proportion2Price1 : Precio 1 de la proporción 2
Proportion3Price1 : Precio 1 de la proporción 3
Proportion4Price1 : Precio 1 de la proporción 4
Proportion5Price1 : Precio 1 de la proporción 5
Proportion6Price1 : Precio 1 de la proporción 6
```

**Proportion7Price1** : Precio 1 de la proporción 7
**Proportion8Price1** : Precio 1 de la proporción 8
**Proportion9Price1:** Precio 1 de la proporción 9
**Proportion2Price2:** Precio 2 de la proporción 2
**Proportion3Price2:** Precio 2 de la proporción 3
**Proportion4Price2:** Precio 2 de la proporción 4
**Proportion5Price2:** Precio 2 de la proporción 5
**Proportion6Price2:** Precio 2 de la proporción 6
**Proportion7Price2:** Precio 2 de la proporción 7
**Proportion8Price2:** Precio 2 de la proporción 8
**Proportion9Price2:** Precio 2 de la proporción 9
**Proportion2Price3:** Precio 3 de la proporción 2
**Proportion3Price3:** Precio 3 de la proporción 3
**Proportion4Price3:** Precio 3 de la proporción 4
**Proportion5Price3:** Precio 3 de la proporción 5
**Proportion6Price3:** Precio 3 de la proporción 6
**Proportion7Price3:** Precio 3 de la proporción 7
**Proportion8Price3:** Precio 3 de la proporción 8
**Proportion9Price3:** Precio 3 de la proporción 9
**Proportion2Price4:** Precio 4 de la proporción 2
**Proportion3Price4:** Precio 4 de la proporción 3
**Proportion4Price4:** Precio 4 de la proporción 4
**Proportion5Price4:** Precio 4 de la proporción 5
**Proportion6Price4:** Precio 4 de la proporción 6
**Proportion7Price4:** Precio 4 de la proporción 7
**Proportion8Price4:** Precio 4 de la proporción 8
**Proportion9Price4:** Precio 4 de la proporción 9
**Proportion2Price5:** Precio 5 de la proporción 2
**Proportion3Price5:** Precio 5 de la proporción 3
**Proportion4Price5:** Precio 5 de la proporción 4
**Proportion5Price5:** Precio 5 de la proporción 5
**Proportion6Price5:** Precio 5 de la proporción 6
**Proportion7Price5:** Precio 5 de la proporción 7
**Proportion8Price5:** Precio 5 de la proporción 8
**Proportion9Price5:** Precio 5 de la proporción 9
**Proportion2PluDiscount:** Estamos descontando el stock de este artículo (si > 0)
**Proportion2PluDiscountDescription:** Descripción del artículo asociado a la proporción 2
**Proportion3PluDiscount:** Estamos descontando el stock de este artículo (si > 0)
**Proportion3PluDiscountDescription:** Descripción del artículo asociado a la proporción 3
**Proportion4PluDiscount:** Estamos descontando el stock de este artículo (si > 0)
**Proportion4PluDiscountDescription:** Descripción del artículo asociado a la proporción 4
**Proportion5PluDiscount:** Estamos descontando el stock de este artículo (si > 0)
**Proportion5PluDiscountDescription:** Descripción del artículo asociado a la proporción 5
**Proportion6PluDiscount:** Estamos descontando el stock de este artículo (si > 0)
**Proportion6PluDiscountDescription:** Descripción del artículo asociado a la proporción 6
**Proportion7PluDiscount:** Estamos descontando el stock de este artículo (si > 0)
**Proportion7PluDiscountDescription:** Descripción del artículo asociado a la proporción 7
**Proportion8PluDiscount:** Estamos descontando el stock de este artículo (si > 0)
**Proportion8PluDiscountDescription:** Descripción del artículo asociado a la proporción 8
**Proportion9PluDiscount:** Estamos descontando el stock de este artículo (si > 0)
**Proportion9PluDiscountDescription:** Descripción del artículo asociado a la proporción 9
**ApplyDiscountsInProp:** Campo lógico que corresponde al campo ‘Aplicar Descuentos Generales del
Artículo en Proporciones’.
**NotAllowedAsInvitation** **_(boolean)_** **:** campo lógico que corresponde al campo ‘No permitir cómo invitación’.


##### JSON de respuesta

###### {

```
"NumberItems": 259,
"ErrorMessage": "",
"ArticlesListData": [
{
"DeptCode": 15,
"DeptDescription": "TABACO",
"AuxPrinters": "",
"MenuDish": false,
"POS_SupplementsProfileID": 0.0,
"SelfOrdering_CommentsProfileID": 0.0,
"SelfOrdering_SupplementsProfileID": 0.0,
"POS_MenuID": 0.0,
"POS_FastfoodID": 0.0,
"POS_PackID": 0.0,
"Commissionable": false,
"ModifiablePrice": false,
"DontPrintTicketValue0": false,
"Weight": false,
"DontNotifyUnitsPrice0": true,
"NotifyModifyPriceUnits": true,
"TwoForOne": false,
"POS_CommentsProfileID": 0.0,
"PriceConfirmation": false,
"FreeDescription": false,
"SAC_Dimensions": null,
"SAC_PricesTable": null,
"Is_Inventoriable": false,
"BuyTAVCode": 0,
"BuyTAVPer": 0.0,
"WebArticle": false,
"TAVCode": 1,
"TAVPer": 10.0,
"ErrorMessage": null,
"ArtDescription": "HABANOS PUROS CJ 10 UNI",
"SaleInfo": {
"Price": 5.0,
"Discount": 0.0,
"PriceWithDiscount": 5.0,
"SalePrice": 0.0,
"SaleDiscount": 0.0,
"SalePriceWithDiscount": 0.0,
"CurrentStock": 0.0,
"OnSale": false,
"PriceToApply": 5.0,
"SalePriceToApply": 0.0
},
"GraphDescrip1": "PUROS",
```

"GraphDescrip2": "HABANOS",
"GraphDescrip3": "",
"ExtendedArtDescription": "",
"ArtCode": 601,
"Alergenos": [],
"Proportion1Description": "",
"Proportion2Active": false,
"Proportion3Active": false,
"Proportion4Active": false,
"Proportion5Active": false,
"Proportion6Active": false,
"Proportion7Active": false,
"Proportion8Active": false,
"Proportion9Active": false,
"Proportion2Amount": 0.0,
"Proportion3Amount": 0.0,
"Proportion4Amount": 0.0,
"Proportion5Amount": 0.0,
"Proportion6Amount": 0.0,
"Proportion7Amount": 0.0,
"Proportion8Amount": 0.0,
"Proportion9Amount": 0.0,
"Proportion2Description": "",
"Proportion3Description": "",
"Proportion4Description": "",
"Proportion5Description": "",
"Proportion6Description": "",
"Proportion7Description": "",
"Proportion8Description": "",
"Proportion9Description": "",
"Proportion2Price1": 0.0,
"Proportion3Price1": 0.0,
"Proportion4Price1": 0.0,
"Proportion5Price1": 0.0,
"Proportion6Price1": 0.0,
"Proportion7Price1": 0.0,
"Proportion8Price1": 0.0,
"Proportion9Price1": 0.0,
"Proportion2Price2": 0.0,
"Proportion3Price2": 0.0,
"Proportion4Price2": 0.0,
"Proportion5Price2": 0.0,
"Proportion6Price2": 0.0,
"Proportion7Price2": 0.0,
"Proportion8Price2": 0.0,
"Proportion9Price2": 0.0,
"Proportion2Price3": 0.0,
"Proportion3Price3": 0.0,
"Proportion4Price3": 0.0,
"Proportion5Price3": 0.0,


"Proportion6Price3": 0.0,
"Proportion7Price3": 0.0,
"Proportion8Price3": 0.0,
"Proportion9Price3": 0.0,
"Proportion2Price4": 0.0,
"Proportion3Price4": 0.0,
"Proportion4Price4": 0.0,
"Proportion5Price4": 0.0,
"Proportion6Price4": 0.0,
"Proportion7Price4": 0.0,
"Proportion8Price4": 0.0,
"Proportion9Price4": 0.0,
"Proportion2Price5": 0.0,
"Proportion3Price5": 0.0,
"Proportion4Price5": 0.0,
"Proportion5Price5": 0.0,
"Proportion6Price5": 0.0,
"Proportion7Price5": 0.0,
"Proportion8Price5": 0.0,
"Proportion9Price5": 0.0,
"Proportion2PluDiscount": 0,
"Proportion2PluDiscountDescription": "",
"Proportion3PluDiscount": 0,
"Proportion3PluDiscountDescription": "",
"Proportion4PluDiscount": 0,
"Proportion4PluDiscountDescription": "",
"Proportion5PluDiscount": 0,
"Proportion5PluDiscountDescription": "",
"Proportion6PluDiscount": 0,
"Proportion6PluDiscountDescription": "",
"Proportion7PluDiscount": 0,
"Proportion7PluDiscountDescription": "",
"Proportion8PluDiscount": 0,
"Proportion8PluDiscountDescription": "",
"Proportion9PluDiscount": 0,
"Proportion9PluDiscountDescription": "",
"ApplyDiscountsInProp": false
},
{
"DeptCode": 1,
"DeptDescription": "REFRESCOS",
"AuxPrinters": "",
"MenuDish": true,
"POS_SupplementsProfileID": 0.0,
"SelfOrdering_CommentsProfileID": 0.0,
"SelfOrdering_SupplementsProfileID": 0.0,
"POS_MenuID": 0.0,
"POS_FastfoodID": 0.0,
"POS_PackID": 0.0,
"Commissionable": false,


"ModifiablePrice": true,
"DontPrintTicketValue0": false,
"Weight": false,
"DontNotifyUnitsPrice0": false,
"NotifyModifyPriceUnits": false,
"TwoForOne": false,
"POS_CommentsProfileID": 0.0,
"PriceConfirmation": false,
"FreeDescription": false,
"SAC_Dimensions": null,
"SAC_PricesTable": null,
"Is_Inventoriable": false,
"BuyTAVCode": 0,
"BuyTAVPer": 0.0,
"WebArticle": true,
"TAVCode": 1,
"TAVPer": 10.0,
"ErrorMessage": null,
"ArtDescription": "COCA-COLA",
"SaleInfo": {
"Price": 1.05,
"Discount": 0.0,
"PriceWithDiscount": 1.05,
"SalePrice": 0.0,
"SaleDiscount": 0.0,
"SalePriceWithDiscount": 0.0,
"CurrentStock": 0.0,
"OnSale": false,
"PriceToApply": 1.05,
"SalePriceToApply": 0.0
},
"GraphDescrip1": "",
"GraphDescrip2": "",
"GraphDescrip3": "",
"ExtendedArtDescription": "",
"ArtCode": 1001,
"Alergenos": [],
"Proportion1Description": "",
"Proportion2Active": false,
"Proportion3Active": false,
"Proportion4Active": false,
"Proportion5Active": false,
"Proportion6Active": false,
"Proportion7Active": false,
"Proportion8Active": false,
"Proportion9Active": false,
"Proportion2Amount": 0.0,
"Proportion3Amount": 0.0,
"Proportion4Amount": 0.0,
"Proportion5Amount": 0.0,


"Proportion6Amount": 0.0,
"Proportion7Amount": 0.0,
"Proportion8Amount": 0.0,
"Proportion9Amount": 0.0,
"Proportion2Description": "",
"Proportion3Description": "",
"Proportion4Description": "",
"Proportion5Description": "",
"Proportion6Description": "",
"Proportion7Description": "",
"Proportion8Description": "",
"Proportion9Description": "",
"Proportion2Price1": 0.0,
"Proportion3Price1": 0.0,
"Proportion4Price1": 0.0,
"Proportion5Price1": 0.0,
"Proportion6Price1": 0.0,
"Proportion7Price1": 0.0,
"Proportion8Price1": 0.0,
"Proportion9Price1": 0.0,
"Proportion2Price2": 0.0,
"Proportion3Price2": 0.0,
"Proportion4Price2": 0.0,
"Proportion5Price2": 0.0,
"Proportion6Price2": 0.0,
"Proportion7Price2": 0.0,
"Proportion8Price2": 0.0,
"Proportion9Price2": 0.0,
"Proportion2Price3": 0.0,
"Proportion3Price3": 0.0,
"Proportion4Price3": 0.0,
"Proportion5Price3": 0.0,
"Proportion6Price3": 0.0,
"Proportion7Price3": 0.0,
"Proportion8Price3": 0.0,
"Proportion9Price3": 0.0,
"Proportion2Price4": 0.0,
"Proportion3Price4": 0.0,
"Proportion4Price4": 0.0,
"Proportion5Price4": 0.0,
"Proportion6Price4": 0.0,
"Proportion7Price4": 0.0,
"Proportion8Price4": 0.0,
"Proportion9Price4": 0.0,
"Proportion2Price5": 0.0,
"Proportion3Price5": 0.0,
"Proportion4Price5": 0.0,
"Proportion5Price5": 0.0,
"Proportion6Price5": 0.0,
"Proportion7Price5": 0.0,


"Proportion8Price5": 0.0,
"Proportion9Price5": 0.0,
"Proportion2PluDiscount": 0,
"Proportion2PluDiscountDescription": "",
"Proportion3PluDiscount": 0,
"Proportion3PluDiscountDescription": "",
"Proportion4PluDiscount": 0,
"Proportion4PluDiscountDescription": "",
"Proportion5PluDiscount": 0,
"Proportion5PluDiscountDescription": "",
"Proportion6PluDiscount": 0,
"Proportion6PluDiscountDescription": "",
"Proportion7PluDiscount": 0,
"Proportion7PluDiscountDescription": "",
"Proportion8PluDiscount": 0,
"Proportion8PluDiscountDescription": "",
"Proportion9PluDiscount": 0,
"Proportion9PluDiscountDescription": "",
"ApplyDiscountsInProp": false
},
{
"DeptCode": 1,
"DeptDescription": "REFRESCOS",
"AuxPrinters": "",
"MenuDish": true,
"POS_SupplementsProfileID": 0.0,
"SelfOrdering_CommentsProfileID": 0.0,
"SelfOrdering_SupplementsProfileID": 0.0,
"POS_MenuID": 0.0,
"POS_FastfoodID": 0.0,
"POS_PackID": 0.0,
"Commissionable": false,
"ModifiablePrice": true,
"DontPrintTicketValue0": false,
"Weight": false,
"DontNotifyUnitsPrice0": false,
"NotifyModifyPriceUnits": false,
"TwoForOne": false,
"POS_CommentsProfileID": 0.0,
"PriceConfirmation": false,
"FreeDescription": false,
"SAC_Dimensions": null,
"SAC_PricesTable": null,
"Is_Inventoriable": false,
"BuyTAVCode": 0,
"BuyTAVPer": 0.0,
"WebArticle": true,
"TAVCode": 3,
"TAVPer": 4.0,
"ErrorMessage": null,


"ArtDescription": "SCHWEPPES LIMÓN",
"SaleInfo": {
"Price": 1.05,
"Discount": 0.0,
"PriceWithDiscount": 1.05,
"SalePrice": 0.0,
"SaleDiscount": 0.0,
"SalePriceWithDiscount": 0.0,
"CurrentStock": 0.0,
"OnSale": false,
"PriceToApply": 1.05,
"SalePriceToApply": 0.0
},
"GraphDescrip1": "",
"GraphDescrip2": "",
"GraphDescrip3": "",
"ExtendedArtDescription": "",
"ArtCode": 1002,
"Alergenos": [],
"Proportion1Description": "",
"Proportion2Active": false,
"Proportion3Active": false,
"Proportion4Active": false,
"Proportion5Active": false,
"Proportion6Active": false,
"Proportion7Active": false,
"Proportion8Active": false,
"Proportion9Active": false,
"Proportion2Amount": 0.0,
"Proportion3Amount": 0.0,
"Proportion4Amount": 0.0,
"Proportion5Amount": 0.0,
"Proportion6Amount": 0.0,
"Proportion7Amount": 0.0,
"Proportion8Amount": 0.0,
"Proportion9Amount": 0.0,
"Proportion2Description": "",
"Proportion3Description": "",
"Proportion4Description": "",
"Proportion5Description": "",
"Proportion6Description": "",
"Proportion7Description": "",
"Proportion8Description": "",
"Proportion9Description": "",
"Proportion2Price1": 0.0,
"Proportion3Price1": 0.0,
"Proportion4Price1": 0.0,
"Proportion5Price1": 0.0,
"Proportion6Price1": 0.0,
"Proportion7Price1": 0.0,


"Proportion8Price1": 0.0,
"Proportion9Price1": 0.0,
"Proportion2Price2": 0.0,
"Proportion3Price2": 0.0,
"Proportion4Price2": 0.0,
"Proportion5Price2": 0.0,
"Proportion6Price2": 0.0,
"Proportion7Price2": 0.0,
"Proportion8Price2": 0.0,
"Proportion9Price2": 0.0,
"Proportion2Price3": 0.0,
"Proportion3Price3": 0.0,
"Proportion4Price3": 0.0,
"Proportion5Price3": 0.0,
"Proportion6Price3": 0.0,
"Proportion7Price3": 0.0,
"Proportion8Price3": 0.0,
"Proportion9Price3": 0.0,
"Proportion2Price4": 0.0,
"Proportion3Price4": 0.0,
"Proportion4Price4": 0.0,
"Proportion5Price4": 0.0,
"Proportion6Price4": 0.0,
"Proportion7Price4": 0.0,
"Proportion8Price4": 0.0,
"Proportion9Price4": 0.0,
"Proportion2Price5": 0.0,
"Proportion3Price5": 0.0,
"Proportion4Price5": 0.0,
"Proportion5Price5": 0.0,
"Proportion6Price5": 0.0,
"Proportion7Price5": 0.0,
"Proportion8Price5": 0.0,
"Proportion9Price5": 0.0,
"Proportion2PluDiscount": 0,
"Proportion2PluDiscountDescription": "",
"Proportion3PluDiscount": 0,
"Proportion3PluDiscountDescription": "",
"Proportion4PluDiscount": 0,
"Proportion4PluDiscountDescription": "",
"Proportion5PluDiscount": 0,
"Proportion5PluDiscountDescription": "",
"Proportion6PluDiscount": 0,
"Proportion6PluDiscountDescription": "",
"Proportion7PluDiscount": 0,
"Proportion7PluDiscountDescription": "",
"Proportion8PluDiscount": 0,
"Proportion8PluDiscountDescription": "",
"Proportion9PluDiscount": 0,
"Proportion9PluDiscountDescription": "",


"ApplyDiscountsInProp": false
},
{
"DeptCode": 1,
"DeptDescription": "REFRESCOS",
"AuxPrinters": "",
"MenuDish": true,
"POS_SupplementsProfileID": 0.0,
"SelfOrdering_CommentsProfileID": 0.0,
"SelfOrdering_SupplementsProfileID": 0.0,
"POS_MenuID": 0.0,
"POS_FastfoodID": 0.0,
"POS_PackID": 0.0,
"Commissionable": false,
"ModifiablePrice": true,
"DontPrintTicketValue0": false,
"Weight": false,
"DontNotifyUnitsPrice0": false,
"NotifyModifyPriceUnits": false,
"TwoForOne": false,
"POS_CommentsProfileID": 0.0,
"PriceConfirmation": false,
"FreeDescription": false,
"SAC_Dimensions": null,
"SAC_PricesTable": null,
"Is_Inventoriable": false,
"BuyTAVCode": 0,
"BuyTAVPer": 0.0,
"WebArticle": false,
"TAVCode": 1,
"TAVPer": 10.0,
"ErrorMessage": null,
"ArtDescription": "SCHWEPPES NARANJA",
"SaleInfo": {
"Price": 1.05,
"Discount": 0.0,
"PriceWithDiscount": 1.05,
"SalePrice": 0.0,
"SaleDiscount": 0.0,
"SalePriceWithDiscount": 0.0,
"CurrentStock": 0.0,
"OnSale": false,
"PriceToApply": 1.05,
"SalePriceToApply": 0.0
},
"GraphDescrip1": "",
"GraphDescrip2": "",
"GraphDescrip3": "",
"ExtendedArtDescription": "",
"ArtCode": 1003,


"Alergenos": [],
"Proportion1Description": "",
"Proportion2Active": false,
"Proportion3Active": false,
"Proportion4Active": false,
"Proportion5Active": false,
"Proportion6Active": false,
"Proportion7Active": false,
"Proportion8Active": false,
"Proportion9Active": false,
"Proportion2Amount": 0.0,
"Proportion3Amount": 0.0,
"Proportion4Amount": 0.0,
"Proportion5Amount": 0.0,
"Proportion6Amount": 0.0,
"Proportion7Amount": 0.0,
"Proportion8Amount": 0.0,
"Proportion9Amount": 0.0,
"Proportion2Description": "",
"Proportion3Description": "",
"Proportion4Description": "",
"Proportion5Description": "",
"Proportion6Description": "",
"Proportion7Description": "",
"Proportion8Description": "",
"Proportion9Description": "",
"Proportion2Price1": 0.0,
"Proportion3Price1": 0.0,
"Proportion4Price1": 0.0,
"Proportion5Price1": 0.0,
"Proportion6Price1": 0.0,
"Proportion7Price1": 0.0,
"Proportion8Price1": 0.0,
"Proportion9Price1": 0.0,
"Proportion2Price2": 0.0,
"Proportion3Price2": 0.0,
"Proportion4Price2": 0.0,
"Proportion5Price2": 0.0,
"Proportion6Price2": 0.0,
"Proportion7Price2": 0.0,
"Proportion8Price2": 0.0,
"Proportion9Price2": 0.0,
"Proportion2Price3": 0.0,
"Proportion3Price3": 0.0,
"Proportion4Price3": 0.0,
"Proportion5Price3": 0.0,
"Proportion6Price3": 0.0,
"Proportion7Price3": 0.0,
"Proportion8Price3": 0.0,
"Proportion9Price3": 0.0,


Proportion2Price4": 0.0,
"Proportion3Price4": 0.0,
"Proportion4Price4": 0.0,
"Proportion5Price4": 0.0,
"Proportion6Price4": 0.0,
"Proportion7Price4": 0.0,
"Proportion8Price4": 0.0,
"Proportion9Price4": 0.0,
"Proportion2Price5": 0.0,
"Proportion3Price5": 0.0,
"Proportion4Price5": 0.0,
"Proportion5Price5": 0.0,
"Proportion6Price5": 0.0,
"Proportion7Price5": 0.0,
"Proportion8Price5": 0.0,
"Proportion9Price5": 0.0,
"Proportion2PluDiscount": 0,
"Proportion2PluDiscountDescription": "",
"Proportion3PluDiscount": 0,
"Proportion3PluDiscountDescription": "",
"Proportion4PluDiscount": 0,
"Proportion4PluDiscountDescription": "",
"Proportion5PluDiscount": 0,
"Proportion5PluDiscountDescription": "",
"Proportion6PluDiscount": 0,
"Proportion6PluDiscountDescription": "",
"Proportion7PluDiscount": 0,
"Proportion7PluDiscountDescription": "",
"Proportion8PluDiscount": 0,
"Proportion8PluDiscountDescription": "",
"Proportion9PluDiscount": 0,
"Proportion9PluDiscountDescription": "",
"ApplyDiscountsInProp": false
},
{
"DeptCode": 1,
"DeptDescription": "REFRESCOS",
"AuxPrinters": "",
"MenuDish": false,
"POS_SupplementsProfileID": 0.0,
"SelfOrdering_CommentsProfileID": 0.0,
"SelfOrdering_SupplementsProfileID": 0.0,
"POS_MenuID": 0.0,
"POS_FastfoodID": 0.0,
"POS_PackID": 0.0,
"Commissionable": false,
"ModifiablePrice": true,
"DontPrintTicketValue0": false,
"Weight": false,
"DontNotifyUnitsPrice0": false,


"NotifyModifyPriceUnits": false,
"TwoForOne": false,
"POS_CommentsProfileID": 0.0,
"PriceConfirmation": false,
"FreeDescription": false,
"SAC_Dimensions": null,
"SAC_PricesTable": null,
"Is_Inventoriable": false,
"BuyTAVCode": 0,
"BuyTAVPer": 0.0,
"WebArticle": false,
"TAVCode": 1,
"TAVPer": 10.0,
"ErrorMessage": null,
"ArtDescription": "CINZANO BITTER",
"SaleInfo": {
"Price": 1.2,
"Discount": 0.0,
"PriceWithDiscount": 1.2,
"SalePrice": 0.0,
"SaleDiscount": 0.0,
"SalePriceWithDiscount": 0.0,
"CurrentStock": 0.0,
"OnSale": false,
"PriceToApply": 1.2,
"SalePriceToApply": 0.0
},
"GraphDescrip1": "",
"GraphDescrip2": "",
"GraphDescrip3": "",
"ExtendedArtDescription": "",
"ArtCode": 1004,
"Alergenos": [],
"Proportion1Description": "",
"Proportion2Active": false,
"Proportion3Active": false,
"Proportion4Active": false,
"Proportion5Active": false,
"Proportion6Active": false,
"Proportion7Active": false,
"Proportion8Active": false,
"Proportion9Active": false,
"Proportion2Amount": 0.0,
"Proportion3Amount": 0.0,
"Proportion4Amount": 0.0,
"Proportion5Amount": 0.0,
"Proportion6Amount": 0.0,
"Proportion7Amount": 0.0,
"Proportion8Amount": 0.0,
"Proportion9Amount": 0.0,


"Proportion2Description": "",
"Proportion3Description": "",
"Proportion4Description": "",
"Proportion5Description": "",
"Proportion6Description": "",
"Proportion7Description": "",
"Proportion8Description": "",
"Proportion9Description": "",
"Proportion2Price1": 0.0,
"Proportion3Price1": 0.0,
"Proportion4Price1": 0.0,
"Proportion5Price1": 0.0,
"Proportion6Price1": 0.0,
"Proportion7Price1": 0.0,
"Proportion8Price1": 0.0,
"Proportion9Price1": 0.0,
"Proportion2Price2": 0.0,
"Proportion3Price2": 0.0,
"Proportion4Price2": 0.0,
"Proportion5Price2": 0.0,
"Proportion6Price2": 0.0,
"Proportion7Price2": 0.0,
"Proportion8Price2": 0.0,
"Proportion9Price2": 0.0,
"Proportion2Price3": 0.0,
"Proportion3Price3": 0.0,
"Proportion4Price3": 0.0,
"Proportion5Price3": 0.0,
"Proportion6Price3": 0.0,
"Proportion7Price3": 0.0,
"Proportion8Price3": 0.0,
"Proportion9Price3": 0.0,
"Proportion2Price4": 0.0,
"Proportion3Price4": 0.0,
"Proportion4Price4": 0.0,
"Proportion5Price4": 0.0,
"Proportion6Price4": 0.0,
"Proportion7Price4": 0.0,
"Proportion8Price4": 0.0,
"Proportion9Price4": 0.0,
"Proportion2Price5": 0.0,
"Proportion3Price5": 0.0,
"Proportion4Price5": 0.0,
"Proportion5Price5": 0.0,
"Proportion6Price5": 0.0,
"Proportion7Price5": 0.0,
"Proportion8Price5": 0.0,
"Proportion9Price5": 0.0,
"Proportion2PluDiscount": 0,
"Proportion2PluDiscountDescription": "",


"Proportion3PluDiscount": 0,
"Proportion3PluDiscountDescription": "",
"Proportion4PluDiscount": 0,
"Proportion4PluDiscountDescription": "",
"Proportion5PluDiscount": 0,
"Proportion5PluDiscountDescription": "",
"Proportion6PluDiscount": 0,
"Proportion6PluDiscountDescription": "",
"Proportion7PluDiscount": 0,
"Proportion7PluDiscountDescription": "",
"Proportion8PluDiscount": 0,
"Proportion8PluDiscountDescription": "",
"Proportion9PluDiscount": 0,
"Proportion9PluDiscountDescription": "",
"ApplyDiscountsInProp": false
},
{
"DeptCode": 1,
"DeptDescription": "REFRESCOS",
"AuxPrinters": "",
"MenuDish": false,
"POS_SupplementsProfileID": 0.0,
"SelfOrdering_CommentsProfileID": 0.0,
"SelfOrdering_SupplementsProfileID": 0.0,
"POS_MenuID": 0.0,
"POS_FastfoodID": 0.0,
"POS_PackID": 0.0,
"Commissionable": false,
"ModifiablePrice": true,
"DontPrintTicketValue0": false,
"Weight": false,
"DontNotifyUnitsPrice0": false,
"NotifyModifyPriceUnits": false,
"TwoForOne": false,
"POS_CommentsProfileID": 0.0,
"PriceConfirmation": false,
"FreeDescription": false,
"SAC_Dimensions": null,
"SAC_PricesTable": null,
"Is_Inventoriable": false,
"BuyTAVCode": 0,
"BuyTAVPer": 0.0,
"WebArticle": false,
"TAVCode": 1,
"TAVPer": 10.0,
"ErrorMessage": null,
"ArtDescription": "PEPSI COLA",
"SaleInfo": {
"Price": 1.05,
"Discount": 0.0,


"PriceWithDiscount": 1.05,
"SalePrice": 0.0,
"SaleDiscount": 0.0,
"SalePriceWithDiscount": 0.0,
"CurrentStock": 0.0,
"OnSale": false,
"PriceToApply": 1.05,
"SalePriceToApply": 0.0
},
"GraphDescrip1": "",
"GraphDescrip2": "",
"GraphDescrip3": "",
"ExtendedArtDescription": "",
"ArtCode": 1005,
"Alergenos": [],
"Proportion1Description": "",
"Proportion2Active": false,
"Proportion3Active": false,
"Proportion4Active": false,
"Proportion5Active": false,
"Proportion6Active": false,
"Proportion7Active": false,
"Proportion8Active": false,
"Proportion9Active": false,
"Proportion2Amount": 0.0,
"Proportion3Amount": 0.0,
"Proportion4Amount": 0.0,
"Proportion5Amount": 0.0,
"Proportion6Amount": 0.0,
"Proportion7Amount": 0.0,
"Proportion8Amount": 0.0,
"Proportion9Amount": 0.0,
"Proportion2Description": "",
"Proportion3Description": "",
"Proportion4Description": "",
"Proportion5Description": "",
"Proportion6Description": "",
"Proportion7Description": "",
"Proportion8Description": "",
"Proportion9Description": "",
"Proportion2Price1": 0.0,
"Proportion3Price1": 0.0,
"Proportion4Price1": 0.0,
"Proportion5Price1": 0.0,
"Proportion6Price1": 0.0,
"Proportion7Price1": 0.0,
"Proportion8Price1": 0.0,
"Proportion9Price1": 0.0,
"Proportion2Price2": 0.0,
"Proportion3Price2": 0.0,


"Proportion4Price2": 0.0,
"Proportion5Price2": 0.0,
"Proportion6Price2": 0.0,
"Proportion7Price2": 0.0,
"Proportion8Price2": 0.0,
"Proportion9Price2": 0.0,
"Proportion2Price3": 0.0,
"Proportion3Price3": 0.0,
"Proportion4Price3": 0.0,
"Proportion5Price3": 0.0,
"Proportion6Price3": 0.0,
"Proportion7Price3": 0.0,
"Proportion8Price3": 0.0,
"Proportion9Price3": 0.0,
"Proportion2Price4": 0.0,
"Proportion3Price4": 0.0,
"Proportion4Price4": 0.0,
"Proportion5Price4": 0.0,
"Proportion6Price4": 0.0,
"Proportion7Price4": 0.0,
"Proportion8Price4": 0.0,
"Proportion9Price4": 0.0,
"Proportion2Price5": 0.0,
"Proportion3Price5": 0.0,
"Proportion4Price5": 0.0,
"Proportion5Price5": 0.0,
"Proportion6Price5": 0.0,
"Proportion7Price5": 0.0,
"Proportion8Price5": 0.0,
"Proportion9Price5": 0.0,
"Proportion2PluDiscount": 0,
"Proportion2PluDiscountDescription": "",
"Proportion3PluDiscount": 0,
"Proportion3PluDiscountDescription": "",
"Proportion4PluDiscount": 0,
"Proportion4PluDiscountDescription": "",
"Proportion5PluDiscount": 0,
"Proportion5PluDiscountDescription": "",
"Proportion6PluDiscount": 0,
"Proportion6PluDiscountDescription": "",
"Proportion7PluDiscount": 0,
"Proportion7PluDiscountDescription": "",
"Proportion8PluDiscount": 0,
"Proportion8PluDiscountDescription": "",
"Proportion9PluDiscount": 0,
"Proportion9PluDiscountDescription": "",
"ApplyDiscountsInProp": false
},
{
"DeptCode": 1,


"DeptDescription": "REFRESCOS",
"AuxPrinters": "",
"MenuDish": false,
"POS_SupplementsProfileID": 0.0,
"SelfOrdering_CommentsProfileID": 0.0,
"SelfOrdering_SupplementsProfileID": 0.0,
"POS_MenuID": 0.0,
"POS_FastfoodID": 0.0,
"POS_PackID": 0.0,
"Commissionable": false,
"ModifiablePrice": true,
"DontPrintTicketValue0": false,
"Weight": false,
"DontNotifyUnitsPrice0": false,
"NotifyModifyPriceUnits": false,
"TwoForOne": false,
"POS_CommentsProfileID": 0.0,
"PriceConfirmation": false,
"FreeDescription": false,
"SAC_Dimensions": null,
"SAC_PricesTable": null,
"Is_Inventoriable": false,
"BuyTAVCode": 0,
"BuyTAVPer": 0.0,
"WebArticle": false,
"TAVCode": 1,
"TAVPer": 10.0,
"ErrorMessage": null,
"ArtDescription": "AGUA MINERAL 1/2 L",
"SaleInfo": {
"Price": 0.6,
"Discount": 0.0,
"PriceWithDiscount": 0.6,
"SalePrice": 0.0,
"SaleDiscount": 0.0,
"SalePriceWithDiscount": 0.0,
"CurrentStock": 0.0,
"OnSale": false,
"PriceToApply": 0.6,
"SalePriceToApply": 0.0
},
"GraphDescrip1": "",
"GraphDescrip2": "",
"GraphDescrip3": "",
"ExtendedArtDescription": "",
"ArtCode": 1006,
"Alergenos": [],
"Proportion1Description": "",
"Proportion2Active": false,
"Proportion3Active": false,


"Proportion4Active": false,
"Proportion5Active": false,
"Proportion6Active": false,
"Proportion7Active": false,
"Proportion8Active": false,
"Proportion9Active": false,
"Proportion2Amount": 0.0,
"Proportion3Amount": 0.0,
"Proportion4Amount": 0.0,
"Proportion5Amount": 0.0,
"Proportion6Amount": 0.0,
"Proportion7Amount": 0.0,
"Proportion8Amount": 0.0,
"Proportion9Amount": 0.0,
"Proportion2Description": "",
"Proportion3Description": "",
"Proportion4Description": "",
"Proportion5Description": "",
"Proportion6Description": "",
"Proportion7Description": "",
"Proportion8Description": "",
"Proportion9Description": "",
"Proportion2Price1": 0.0,
"Proportion3Price1": 0.0,
"Proportion4Price1": 0.0,
"Proportion5Price1": 0.0,
"Proportion6Price1": 0.0,
"Proportion7Price1": 0.0,
"Proportion8Price1": 0.0,
"Proportion9Price1": 0.0,
"Proportion2Price2": 0.0,
"Proportion3Price2": 0.0,
"Proportion4Price2": 0.0,
"Proportion5Price2": 0.0,
"Proportion6Price2": 0.0,
"Proportion7Price2": 0.0,
"Proportion8Price2": 0.0,
"Proportion9Price2": 0.0,
"Proportion2Price3": 0.0,
"Proportion3Price3": 0.0,
"Proportion4Price3": 0.0,
"Proportion5Price3": 0.0,
"Proportion6Price3": 0.0,
"Proportion7Price3": 0.0,
"Proportion8Price3": 0.0,
"Proportion9Price3": 0.0,
"Proportion2Price4": 0.0,
"Proportion3Price4": 0.0,
"Proportion4Price4": 0.0,
"Proportion5Price4": 0.0,


"Proportion6Price4": 0.0,
"Proportion7Price4": 0.0,
"Proportion8Price4": 0.0,
"Proportion9Price4": 0.0,
"Proportion2Price5": 0.0,
"Proportion3Price5": 0.0,
"Proportion4Price5": 0.0,
"Proportion5Price5": 0.0,
"Proportion6Price5": 0.0,
"Proportion7Price5": 0.0,
"Proportion8Price5": 0.0,
"Proportion9Price5": 0.0,
"Proportion2PluDiscount": 0,
"Proportion2PluDiscountDescription": "",
"Proportion3PluDiscount": 0,
"Proportion3PluDiscountDescription": "",
"Proportion4PluDiscount": 0,
"Proportion4PluDiscountDescription": "",
"Proportion5PluDiscount": 0,
"Proportion5PluDiscountDescription": "",
"Proportion6PluDiscount": 0,
"Proportion6PluDiscountDescription": "",
"Proportion7PluDiscount": 0,
"Proportion7PluDiscountDescription": "",
"Proportion8PluDiscount": 0,
"Proportion8PluDiscountDescription": "",
"Proportion9PluDiscount": 0,
"Proportion9PluDiscountDescription": "",
"ApplyDiscountsInProp": false
},
{
"DeptCode": 1,
"DeptDescription": "REFRESCOS",
"AuxPrinters": "",
"MenuDish": false,
"POS_SupplementsProfileID": 0.0,
"SelfOrdering_CommentsProfileID": 0.0,
"SelfOrdering_SupplementsProfileID": 0.0,
"POS_MenuID": 0.0,
"POS_FastfoodID": 0.0,
"POS_PackID": 0.0,
"Commissionable": false,
"ModifiablePrice": true,
"DontPrintTicketValue0": false,
"Weight": false,
"DontNotifyUnitsPrice0": false,
"NotifyModifyPriceUnits": false,
"TwoForOne": false,
"POS_CommentsProfileID": 0.0,
"PriceConfirmation": false,


"FreeDescription": false,
"SAC_Dimensions": null,
"SAC_PricesTable": null,
"Is_Inventoriable": false,
"BuyTAVCode": 0,
"BuyTAVPer": 0.0,
"WebArticle": false,
"TAVCode": 1,
"TAVPer": 10.0,
"ErrorMessage": null,
"ArtDescription": "ZUMO DE MELOCOTON",
"SaleInfo": {
"Price": 1.2,
"Discount": 0.0,
"PriceWithDiscount": 1.2,
"SalePrice": 0.0,
"SaleDiscount": 0.0,
"SalePriceWithDiscount": 0.0,
"CurrentStock": 0.0,
"OnSale": false,
"PriceToApply": 1.2,
"SalePriceToApply": 0.0
},
"GraphDescrip1": "",
"GraphDescrip2": "",
"GraphDescrip3": "",
"ExtendedArtDescription": "",
"ArtCode": 1007,
"Alergenos": [],
"Proportion1Description": "",
"Proportion2Active": false,
"Proportion3Active": false,
"Proportion4Active": false,
"Proportion5Active": false,
"Proportion6Active": false,
"Proportion7Active": false,
"Proportion8Active": false,
"Proportion9Active": false,
"Proportion2Amount": 0.0,
"Proportion3Amount": 0.0,
"Proportion4Amount": 0.0,
"Proportion5Amount": 0.0,
"Proportion6Amount": 0.0,
"Proportion7Amount": 0.0,
"Proportion8Amount": 0.0,
"Proportion9Amount": 0.0,
"Proportion2Description": "",
"Proportion3Description": "",
"Proportion4Description": "",
"Proportion5Description": "",


"Proportion6Description": "",
"Proportion7Description": "",
"Proportion8Description": "",
"Proportion9Description": "",
"Proportion2Price1": 0.0,
"Proportion3Price1": 0.0,
"Proportion4Price1": 0.0,
"Proportion5Price1": 0.0,
"Proportion6Price1": 0.0,
"Proportion7Price1": 0.0,
"Proportion8Price1": 0.0,
"Proportion9Price1": 0.0,
"Proportion2Price2": 0.0,
"Proportion3Price2": 0.0,
"Proportion4Price2": 0.0,
"Proportion5Price2": 0.0,
"Proportion6Price2": 0.0,
"Proportion7Price2": 0.0,
"Proportion8Price2": 0.0,
"Proportion9Price2": 0.0,
"Proportion2Price3": 0.0,
"Proportion3Price3": 0.0,
"Proportion4Price3": 0.0,
"Proportion5Price3": 0.0,
"Proportion6Price3": 0.0,
"Proportion7Price3": 0.0,
"Proportion8Price3": 0.0,
"Proportion9Price3": 0.0,
"Proportion2Price4": 0.0,
"Proportion3Price4": 0.0,
"Proportion4Price4": 0.0,
"Proportion5Price4": 0.0,
"Proportion6Price4": 0.0,
"Proportion7Price4": 0.0,
"Proportion8Price4": 0.0,
"Proportion9Price4": 0.0,
"Proportion2Price5": 0.0,
"Proportion3Price5": 0.0,
"Proportion4Price5": 0.0,
"Proportion5Price5": 0.0,
"Proportion6Price5": 0.0,
"Proportion7Price5": 0.0,
"Proportion8Price5": 0.0,
"Proportion9Price5": 0.0,
"Proportion2PluDiscount": 0,
"Proportion2PluDiscountDescription": "",
"Proportion3PluDiscount": 0,
"Proportion3PluDiscountDescription": "",
"Proportion4PluDiscount": 0,
"Proportion4PluDiscountDescription": "",


"Proportion5PluDiscount": 0,
"Proportion5PluDiscountDescription": "",
"Proportion6PluDiscount": 0,
"Proportion6PluDiscountDescription": "",
"Proportion7PluDiscount": 0,
"Proportion7PluDiscountDescription": "",
"Proportion8PluDiscount": 0,
"Proportion8PluDiscountDescription": "",
"Proportion9PluDiscount": 0,
"Proportion9PluDiscountDescription": "",
"ApplyDiscountsInProp": false
},
{
"DeptCode": 1,
"DeptDescription": "REFRESCOS",
"AuxPrinters": "",
"MenuDish": false,
"POS_SupplementsProfileID": 0.0,
"SelfOrdering_CommentsProfileID": 0.0,
"SelfOrdering_SupplementsProfileID": 0.0,
"POS_MenuID": 0.0,
"POS_FastfoodID": 0.0,
"POS_PackID": 0.0,
"Commissionable": false,
"ModifiablePrice": true,
"DontPrintTicketValue0": false,
"Weight": false,
"DontNotifyUnitsPrice0": false,
"NotifyModifyPriceUnits": false,
"TwoForOne": false,
"POS_CommentsProfileID": 0.0,
"PriceConfirmation": false,
"FreeDescription": false,
"SAC_Dimensions": null,
"SAC_PricesTable": null,
"Is_Inventoriable": false,
"BuyTAVCode": 0,
"BuyTAVPer": 0.0,
"WebArticle": false,
"TAVCode": 1,
"TAVPer": 10.0,
"ErrorMessage": null,
"ArtDescription": "ZUMO DE PIÑA",
"SaleInfo": {
"Price": 1.2,
"Discount": 0.0,
"PriceWithDiscount": 1.2,
"SalePrice": 0.0,
"SaleDiscount": 0.0,
"SalePriceWithDiscount": 0.0,


"CurrentStock": 0.0,
"OnSale": false,
"PriceToApply": 1.2,
"SalePriceToApply": 0.0
},
"GraphDescrip1": "",
"GraphDescrip2": "",
"GraphDescrip3": "",
"ExtendedArtDescription": "",
"ArtCode": 1008,
"Alergenos": [],
"Proportion1Description": "",
"Proportion2Active": false,
"Proportion3Active": false,
"Proportion4Active": false,
"Proportion5Active": false,
"Proportion6Active": false,
"Proportion7Active": false,
"Proportion8Active": false,
"Proportion9Active": false,
"Proportion2Amount": 0.0,
"Proportion3Amount": 0.0,
"Proportion4Amount": 0.0,
"Proportion5Amount": 0.0,
"Proportion6Amount": 0.0,
"Proportion7Amount": 0.0,
"Proportion8Amount": 0.0,
"Proportion9Amount": 0.0,
"Proportion2Description": "",
"Proportion3Description": "",
"Proportion4Description": "",
"Proportion5Description": "",
"Proportion6Description": "",
"Proportion7Description": "",
"Proportion8Description": "",
"Proportion9Description": "",
"Proportion2Price1": 0.0,
"Proportion3Price1": 0.0,
"Proportion4Price1": 0.0,
"Proportion5Price1": 0.0,
"Proportion6Price1": 0.0,
"Proportion7Price1": 0.0,
"Proportion8Price1": 0.0,
"Proportion9Price1": 0.0,
"Proportion2Price2": 0.0,
"Proportion3Price2": 0.0,
"Proportion4Price2": 0.0,
"Proportion5Price2": 0.0,
"Proportion6Price2": 0.0,
"Proportion7Price2": 0.0,


"Proportion8Price2": 0.0,
"Proportion9Price2": 0.0,
"Proportion2Price3": 0.0,
"Proportion3Price3": 0.0,
"Proportion4Price3": 0.0,
"Proportion5Price3": 0.0,
"Proportion6Price3": 0.0,
"Proportion7Price3": 0.0,
"Proportion8Price3": 0.0,
"Proportion9Price3": 0.0,
"Proportion2Price4": 0.0,
"Proportion3Price4": 0.0,
"Proportion4Price4": 0.0,
"Proportion5Price4": 0.0,
"Proportion6Price4": 0.0,
"Proportion7Price4": 0.0,
"Proportion8Price4": 0.0,
"Proportion9Price4": 0.0,
"Proportion2Price5": 0.0,
"Proportion3Price5": 0.0,
"Proportion4Price5": 0.0,
"Proportion5Price5": 0.0,
"Proportion6Price5": 0.0,
"Proportion7Price5": 0.0,
"Proportion8Price5": 0.0,
"Proportion9Price5": 0.0,
"Proportion2PluDiscount": 0,
"Proportion2PluDiscountDescription": "",
"Proportion3PluDiscount": 0,
"Proportion3PluDiscountDescription": "",
"Proportion4PluDiscount": 0,
"Proportion4PluDiscountDescription": "",
"Proportion5PluDiscount": 0,
"Proportion5PluDiscountDescription": "",
"Proportion6PluDiscount": 0,
"Proportion6PluDiscountDescription": "",
"Proportion7PluDiscount": 0,
"Proportion7PluDiscountDescription": "",
"Proportion8PluDiscount": 0,
"Proportion8PluDiscountDescription": "",
"Proportion9PluDiscount": 0,
"Proportion9PluDiscountDescription": "",
"ApplyDiscountsInProp": false
},
{
"DeptCode": 1,
"DeptDescription": "REFRESCOS",
"AuxPrinters": "",
"MenuDish": false,
"POS_SupplementsProfileID": 0.0,


"SelfOrdering_CommentsProfileID": 0.0,
"SelfOrdering_SupplementsProfileID": 0.0,
"POS_MenuID": 0.0,
"POS_FastfoodID": 0.0,
"POS_PackID": 0.0,
"Commissionable": false,
"ModifiablePrice": true,
"DontPrintTicketValue0": false,
"Weight": false,
"DontNotifyUnitsPrice0": false,
"NotifyModifyPriceUnits": false,
"TwoForOne": false,
"POS_CommentsProfileID": 0.0,
"PriceConfirmation": false,
"FreeDescription": false,
"SAC_Dimensions": null,
"SAC_PricesTable": null,
"Is_Inventoriable": false,
"BuyTAVCode": 0,
"BuyTAVPer": 0.0,
"WebArticle": false,
"TAVCode": 1,
"TAVPer": 10.0,
"ErrorMessage": null,
"ArtDescription": "ZUMO DE NARANJA",
"SaleInfo": {
"Price": 1.2,
"Discount": 0.0,
"PriceWithDiscount": 1.2,
"SalePrice": 0.0,
"SaleDiscount": 0.0,
"SalePriceWithDiscount": 0.0,
"CurrentStock": 0.0,
"OnSale": false,
"PriceToApply": 1.2,
"SalePriceToApply": 0.0
},
"GraphDescrip1": "",
"GraphDescrip2": "",
"GraphDescrip3": "",
"ExtendedArtDescription": "",
"ArtCode": 1009,
"Alergenos": [],
"Proportion1Description": "",
"Proportion2Active": false,
"Proportion3Active": false,
"Proportion4Active": false,
"Proportion5Active": false,
"Proportion6Active": false,
"Proportion7Active": false,


"Proportion8Active": false,
"Proportion9Active": false,
"Proportion2Amount": 0.0,
"Proportion3Amount": 0.0,
"Proportion4Amount": 0.0,
"Proportion5Amount": 0.0,
"Proportion6Amount": 0.0,
"Proportion7Amount": 0.0,
"Proportion8Amount": 0.0,
"Proportion9Amount": 0.0,
"Proportion2Description": "",
"Proportion3Description": "",
"Proportion4Description": "",
"Proportion5Description": "",
"Proportion6Description": "",
"Proportion7Description": "",
"Proportion8Description": "",
"Proportion9Description": "",
"Proportion2Price1": 0.0,
"Proportion3Price1": 0.0,
"Proportion4Price1": 0.0,
"Proportion5Price1": 0.0,
"Proportion6Price1": 0.0,
"Proportion7Price1": 0.0,
"Proportion8Price1": 0.0,
"Proportion9Price1": 0.0,
"Proportion2Price2": 0.0,
"Proportion3Price2": 0.0,
"Proportion4Price2": 0.0,
"Proportion5Price2": 0.0,
"Proportion6Price2": 0.0,
"Proportion7Price2": 0.0,
"Proportion8Price2": 0.0,
"Proportion9Price2": 0.0,
"Proportion2Price3": 0.0,
"Proportion3Price3": 0.0,
"Proportion4Price3": 0.0,
"Proportion5Price3": 0.0,
"Proportion6Price3": 0.0,
"Proportion7Price3": 0.0,
"Proportion8Price3": 0.0,
"Proportion9Price3": 0.0,
"Proportion2Price4": 0.0,
"Proportion3Price4": 0.0,
"Proportion4Price4": 0.0,
"Proportion5Price4": 0.0,
"Proportion6Price4": 0.0,
"Proportion7Price4": 0.0,
"Proportion8Price4": 0.0,
"Proportion9Price4": 0.0,


"Proportion2Price5": 0.0,
"Proportion3Price5": 0.0,
"Proportion4Price5": 0.0,
"Proportion5Price5": 0.0,
"Proportion6Price5": 0.0,
"Proportion7Price5": 0.0,
"Proportion8Price5": 0.0,
"Proportion9Price5": 0.0,
"Proportion2PluDiscount": 0,
"Proportion2PluDiscountDescription": "",
"Proportion3PluDiscount": 0,
"Proportion3PluDiscountDescription": "",
"Proportion4PluDiscount": 0,
"Proportion4PluDiscountDescription": "",
"Proportion5PluDiscount": 0,
"Proportion5PluDiscountDescription": "",
"Proportion6PluDiscount": 0,
"Proportion6PluDiscountDescription": "",
"Proportion7PluDiscount": 0,
"Proportion7PluDiscountDescription": "",
"Proportion8PluDiscount": 0,
"Proportion8PluDiscountDescription": "",
"Proportion9PluDiscount": 0,
"Proportion9PluDiscountDescription": "",
"ApplyDiscountsInProp": false
}
]
}


### CreateArticlesAndUpdateProfiles

Mediante este comando se podrá dar de alta un artículo ya existente a partir de unos datos de entrada y, si procede, dará de alta
los datos del artículo en una serie de perfiles de terminales.

##### Ruta de llamada

/API/Articles/CreateAndUpdateProfiles

##### Parámetros de entrada

**AutomaticCode(boolean):** Campo que en caso de establecerse a true, hará que el número del artículo a ser creado, sea suministrado
de manera automática ignorando el contenido del campo **_ArtCode_**. Si se establece a false, al artículo creado se le establecerá el
código de artículo especificado en el campo **_ArtCode_**.

**ArticleData(** **_ArticleListDataType_** **)** , Colección de objetos **_ArticleListDataType_** con la información para la creación del artículo. Tiene
una estructura del tipo **ArticleListDataTye** :

```
DeptCode (integer) : Valor con el código de departamento de venta asociado al artículo.
```
```
DeptDescription (string) : Cadena con la descripción del departamento de venta asociado al artículo.
```
```
MenuDish (boolean) : Campo lógico que corresponde con el campo ‘Plato de Menú’ del artículo.
```
```
WebArticle (boolean) : Campo lógico que corresponde con el campo ‘Artículo Web’ del artículo.
```
```
POS_SupplementsProfileID (decimal) : Valor con el código de perfil de suplementos asociado al artículo.
```
```
SelfOrdering_CommentsProfileID (decimal) : Valor con el código de perfil de comentarios asociado a la configuración de
autocomanda del artículo.
```
```
SelfOrdering_SupplementsProfileID (decimal) : Valor con el código de perfil de suplementos asociado a la configuración de
autocomanda del artículo.
```
```
POS_MenuID (decimal) : Valor con el código de definición de menú asociado al artículo.
```
```
POS_FastfoodID (decimal) : Valor con el código de definición fastfood asociado al artículo.
```
```
POS_PackID (decimal) : Valor con el código de definición de pack asociado al artículo.
```
```
Is_Inventoriable (boolean) : Campo lógico que corresponde con el campo ‘Inventariable’ del artículo.
```
```
BuyTAVCode (integer) : Valor con el código de IVA de compra asociado al artículo.
```
```
BuyTAVPer (decimal) : Valor decimal con el porcentaje del IVA de compra asociado al artículo.
```
```
TAVCode (integer) : Valor con el código de IVA de venta asociado al artículo.
```
```
TAVPer (decimal) : Valor decimal con el porcentaje del IVA de venta asociado al artículo.
```
```
AuxPrinters (string) : Cadena con la secuencia de impresoras auxiliares asociadas al artículo.
```
```
Commissionable (boolean) : Campo lógico que corresponde con el campo ‘Artículo Comisionable’ del artículo.
```
```
ModifiablePrice (boolean) : Campo lógico que corresponde con el campo ‘Precio Modificable’ del artículo.
```

**DontPrintTicketValue0** **_(boolean)_** : Campo lógico que corresponde con el campo ‘No Imprimir en Factura si Valor 0’ del
artículo.

**Weight** **_(boolean)_** : Campo lógico que corresponde con el campo ‘Artículo de Peso’ del artículo.

**DontNotifyUnitsPrice0** **_(boolean)_** : Campo lógico que corresponde con el campo ‘No Avisar si unidades o Precio es 0’ del
artículo.

**NotifyModifyPriceUnits** **_(boolean)_** : Campo lógico que corresponde con el campo ‘Avisar para Modificar Precio y/o
Unidades’ del artículo.

**TwoForOne** **_(boolean)_** : Campo lógico que corresponde con el campo ‘Artículo Dos x Uno” del artículo.

**POS_CommentsProfileID (decimal)** : Valor con el código de perfil de comentarios asociado al artículo.

**ErrorMessage** **_(string)_** : Cadena que almacenará posibles errores. Si la consulta se realizó correctamente, estará vacía.

**PriceConfirmation** **_(boolean)_** : Campo lógico que corresponde con el campo ‘Confirmación Precio’ del artículo.

**IsCombinable** **_(boolean)_** : Campo lógico que indica si el artículo es combinable.

**CombinedDescription** **_(string)_** : Cadena de texto que corresponde a la descripción del combinado (máx. 15 caracteres).

**CombBasePrice1** **_(decimal)_** : Valor decimal del precio base 1 del combinado.

**CombBasePrice2** **_(decimal)_** : Valor decimal del precio base 2 del combinado.

**CombBasePrice3** **_(decimal)_** : Valor decimal del precio base 3 del combinado.

**CombBasePrice4** **_(decimal)_** : Valor decimal del precio base 4 del combinado.

**CombBasePrice5** **_(decimal)_** : Valor decimal del precio base 5 del combinado.

**CombAuxPrice1** **_(decimal)_** : Valor decimal del precio auxiliar 1 del combinado.

**CombAuxPrice2** **_(decimal)_** : Valor decimal del precio auxiliar 2 del combinado.

**CombAuxPrice3** **_(decimal)_** : Valor decimal del precio auxiliar 3 del combinado.

**CombAuxPrice4** **_(decimal)_** : Valor decimal del precio auxiliar 4 del combinado.

**CombAuxPrice5** **_(decimal)_** : Valor decimal del precio auxiliar 5 del combinado.

**ActivateAlwaysCombined** **_(boolean)_** : Campo lógico que corresponde al campo ‘Activar Siempre Combinado’.

**MandatoryCombined** **_(integer)_** : Valor entero que indica la Obligatoriedad de Combinado: 1: Obligatoriedad de Base; 2:
Obligatoriedad de Auxiliar; 3: Indiferente.

**CombinedAssocType** **_(integer)_** : Valor entero que corresponde al campo ‘Departamentos Asociados al Combinado’: 1: No
Asociar; 2: Asociar a un Departamento; 3: Asociar a una Maxi-Pantalla.

**CombinedDepartmentAssoc** **_(integer)_** : Valor entero que indica el código de Departamento Asociado a los combinados.

**CombinedDepartmentAssocDescription** **_(string)_** : Cadena de texto que corresponde a la descripción del Departamento
Asociado a los combinados (máx. 40 caracteres).

**CombinedMaxiscreenAssoc** **_(integer)_** : Valor entero que indica el código de Maxipantalla Asociada a los combinados.


**CombinedMaxiscreenAssocDescription** **_(string)_** : Cadena de texto que corresponde a la descripción de la Maxipantalla
Asociada a los combinados (máx. 40 caracteres).

**ApplyDiscountsInComb** **_(boolean)_** : Campo lógico que corresponde al campo ‘Aplicar Descuentos Generales del Artículo en
Combinados’.

**FreeDescription** **_(boolean)_** : Campo lógico que corresponde con el campo ‘Descripción Libre’ del artículo.

**ArtDescription** **_(string)_** : Cadena de texto que corresponde a la descripción del artículo (máx. 40 caracteres).

**Price1** **_(decimal)_** : Valor decimal con el PVP1 del artículo.

**Price2** **_(decimal)_** : Valor decimal con el PVP2 del artículo.

**Price3** **_(decimal)_** : Valor decimal con el PVP3 del artículo.

**Price4** **_(decimal)_** : Valor decimal con el PVP4 del artículo.

**Price5** **_(decimal)_** : Valor decimal con el PVP5 del artículo.

**Dct1** **_(decimal)_** : Valor decimal con el porcentaje de descuento 1 del artículo.

**Dct2** **_(decimal)_** : Valor decimal con el porcentaje de descuento 2 del artículo.

**Dct3** **_(decimal)_** : Valor decimal con el porcentaje de descuento 3 del artículo.

**Dct4** **_(decimal)_** : Valor decimal con el porcentaje de descuento 4 del artículo.

**Dct5** **_(decimal)_** : Valor decimal con el porcentaje de descuento 5 del artículo.

**GraphDescrip1** **_(string)_** : Cadena de texto que corresponde a la Descripción Gráfica de la Línea 1 del artículo.

**GraphDescrip2** **_(string)_** : Cadena de texto que corresponde a la Descripción Gráfica de la Línea 2 del artículo.

**GraphDescrip3** **_(string)_** : Cadena de texto que corresponde a la Descripción Gráfica de la Línea 3 del artículo.

**ExtendedArtDescription** **_(string)_** : Cadena de texto que corresponde a la Descripción Extendida del artículo.

**Proportion1Description** **_(string)_** : Cadena de texto que corresponde al campo ‘Descripción de la 1ª Proporción’ (máx. 40
caracteres).

**Proportion2Active** **_(boolean)_** : Campo lógico que corresponde al campo ‘Activar 2ª Proporción’.

**Proportion3Active** **_(boolean)_** : Campo lógico que corresponde al campo ‘Activar 3ª Proporción’.

**Proportion4Active** **_(boolean)_** : Campo lógico que corresponde al campo ‘Activar 4ª Proporción’.

**Proportion5Active** **_(boolean)_** : Campo lógico que corresponde al campo ‘Activar 5ª Proporción’.

**Proportion6Active** **_(boolean)_** : Campo lógico que corresponde al campo ‘Activar 6ª Proporción’.

**Proportion7Active** **_(boolean)_** : Campo lógico que corresponde al campo ‘Activar 7ª Proporción’.

**Proportion8Active** **_(boolean)_** : Campo lógico que corresponde al campo ‘Activar 8ª Proporción’.

**Proportion9Active** **_(boolean)_** : Campo lógico que corresponde al campo ‘Activar 9ª Proporción’.


**Proportion2Amount** **_(decimal)_** : Valor decimal correspondiente al campo ‘Cantidad 2ª Proporción’.

**Proportion3Amount** **_(decimal)_** : Valor decimal correspondiente al campo ‘Cantidad 3ª Proporción’.

**Proportion4Amount** **_(decimal)_** : Valor decimal correspondiente al campo ‘Cantidad 4ª Proporción’.

**Proportion5Amount** **_(decimal)_** : Valor decimal correspondiente al campo ‘Cantidad 5ª Proporción’.

**Proportion6Amount** **_(decimal)_** : Valor decimal correspondiente al campo ‘Cantidad 6ª Proporción’.

**Proportion7Amount** **_(decimal)_** : Valor decimal correspondiente al campo ‘Cantidad 7ª Proporción’.

**Proportion8Amount** **_(decimal)_** : Valor decimal correspondiente al campo ‘Cantidad 8ª Proporción’.

**Proportion9Amount** **_(decimal)_** : Valor decimal correspondiente al campo ‘Cantidad 9ª Proporción’.

**Proportion2Description** **_(string)_** : Cadena de texto que corresponde al campo ‘Descripción de la 2ª Proporción’ (máx. 40
caracteres).

**Proportion3Description** **_(string)_** : Cadena de texto que corresponde al campo ‘Descripción de la 3ª Proporción’ (máx. 40
caracteres).

**Proportion4Description** **_(string)_** : Cadena de texto que corresponde al campo ‘Descripción de la 4ª Proporción’ (máx. 40
caracteres).

**Proportion5Description** **_(string)_** : Cadena de texto que corresponde al campo ‘Descripción de la 5ª Proporción’ (máx. 40
caracteres).

**Proportion6Description** **_(string)_** : Cadena de texto que corresponde al campo ‘Descripción de la 6ª Proporción’ (máx. 40
caracteres).

**Proportion7Description** **_(string)_** : Cadena de texto que corresponde al campo ‘Descripción de la 7ª Proporción’ (máx. 40
caracteres).

**Proportion8Description** **_(string)_** : Cadena de texto que corresponde al campo ‘Descripción de la 8ª Proporción’ (máx. 40
caracteres).

##### Proportion9Description (string) : Cadena de texto que corresponde al campo ‘Descripción de la 9ª Proporción’

##### (máx. 40 caracteres).

**Proportion2Price1** **_(decimal)_** : Valor decimal con el PVP1 de la 2ª Proporción.

**Proportion3Price1** **_(decimal)_** : Valor decimal con el PVP1 de la 3ª Proporción.

**Proportion4Price1** **_(decimal)_** : Valor decimal con el PVP1 de la 4ª Proporción.

**Proportion5Price1** **_(decimal)_** : Valor decimal con el PVP1 de la 5ª Proporción.

**Proportion6Price1** **_(decimal)_** : Valor decimal con el PVP1 de la 6ª Proporción.

**Proportion7Price1** **_(decimal)_** : Valor decimal con el PVP1 de la 7ª Proporción.

**Proportion8Price1** **_(decimal)_** : Valor decimal con el PVP1 de la 8ª Proporción.

**Proportion9Price1** **_(decimal)_** : Valor decimal con el PVP1 de la 9ª Proporción.

**Proportion2Price2** **_(decimal)_** : Valor decimal con el PVP2 de la 2ª Proporción.


**Proportion3Price2** **_(decimal)_** : Valor decimal con el PVP2 de la 3ª Proporción.

**Proportion4Price2** **_(decimal)_** : Valor decimal con el PVP2 de la 4ª Proporción.

**Proportion5Price2** **_(decimal)_** : Valor decimal con el PVP2 de la 5ª Proporción.

**Proportion6Price2** **_(decimal)_** : Valor decimal con el PVP2 de la 6ª Proporción.

**Proportion7Price2** **_(decimal)_** : Valor decimal con el PVP2 de la 7ª Proporción.

**Proportion8Price2** **_(decimal)_** : Valor decimal con el PVP2 de la 8ª Proporción.

**Proportion9Price2** **_(decimal)_** : Valor decimal con el PVP2 de la 9ª Proporción.

**Proportion2Price3** **_(decimal)_** : Valor decimal con el PVP3 de la 2ª Proporción.

**Proportion3Price3** **_(decimal)_** : Valor decimal con el PVP3 de la 3ª Proporción.

**Proportion4Price3** **_(decimal)_** : Valor decimal con el PVP3 de la 4ª Proporción.

**Proportion5Price3** **_(decimal)_** : Valor decimal con el PVP3 de la 5ª Proporción.

**Proportion6Price3** **_(decimal)_** : Valor decimal con el PVP3 de la 6ª Proporción.

**Proportion7Price3** **_(decimal)_** : Valor decimal con el PVP3 de la 7ª Proporción.

**Proportion8Price3** **_(decimal)_** : Valor decimal con el PVP3 de la 8ª Proporción.

**Proportion9Price3** **_(decimal)_** : Valor decimal con el PVP3 de la 9ª Proporción.

**Proportion2Price4** **_(decimal)_** : Valor decimal con el PVP4 de la 2ª Proporción.

**Proportion3Price4** **_(decimal)_** : Valor decimal con el PVP4 de la 3ª Proporción.

**Proportion4Price4** **_(decimal)_** : Valor decimal con el PVP4 de la 4ª Proporción.

**Proportion5Price4** **_(decimal)_** : Valor decimal con el PVP4 de la 5ª Proporción.

**Proportion6Price4** **_(decimal)_** : Valor decimal con el PVP4 de la 6ª Proporción.

**Proportion7Price4** **_(decimal)_** : Valor decimal con el PVP4 de la 7ª Proporción.

**Proportion8Price4** **_(decimal)_** : Valor decimal con el PVP4 de la 8ª Proporción.

**Proportion9Price4** **_(decimal)_** : Valor decimal con el PVP4 de la 9ª Proporción.

**Proportion2Price5** **_(decimal)_** : Valor decimal con el PVP5 de la 2ª Proporción.

**Proportion3Price5** **_(decimal)_** : Valor decimal con el PVP5 de la 3ª Proporción.

**Proportion4Price5** **_(decimal)_** : Valor decimal con el PVP5 de la 4ª Proporción.

**Proportion5Price5** **_(decimal)_** : Valor decimal con el PVP5 de la 5ª Proporción.

**Proportion6Price5** **_(decimal)_** : Valor decimal con el PVP5 de la 6ª Proporción.

**Proportion7Price5** **_(decimal)_** : Valor decimal con el PVP5 de la 7ª Proporción.


**Proportion8Price5** **_(decimal)_** : Valor decimal con el PVP5 de la 8ª Proporción.

**Proportion9Price5** **_(decimal)_** : Valor decimal con el PVP5 de la 9ª Proporción.

**Proportion2PluDiscount** **_(integer)_** : Valor entero que corresponde con el campo ‘PLU Descontar de la 2ª Proporción’.

**Proportion2PluDiscountDescription** **_(string)_** : Cadena de texto que corresponde al campo de la descripción del ‘PLU
Descontar de la 2ª Proporción’.

**Proportion3PluDiscount** **_(integer)_** : Valor entero que corresponde con el campo ‘PLU Descontar de la 3ª Proporción’.

**Proportion3PluDiscountDescription** **_(string)_** : Cadena de texto que corresponde al campo de la descripción del ‘PLU
Descontar de la 3ª Proporción’.

**Proportion4PluDiscount** **_(integer)_** : Valor entero que corresponde con el campo ‘PLU Descontar de la 4ª Proporción’.

**Proportion4PluDiscountDescription** **_(string)_** : Cadena de texto que corresponde al campo de la descripción del ‘PLU
Descontar de la 4ª Proporción’.

**Proportion5PluDiscount** **_(integer)_** : Valor entero que corresponde con el campo ‘PLU Descontar de la 5ª Proporción’.

**Proportion5PluDiscountDescription** **_(string)_** : Cadena de texto que corresponde al campo de la descripción del ‘PLU
Descontar de la 5ª Proporción’.

**Proportion6PluDiscount** **_(integer)_** : Valor entero que corresponde con el campo ‘PLU Descontar de la 6ª Proporción’.

**Proportion6PluDiscountDescription** **_(string)_** : Cadena de texto que corresponde al campo de la descripción del ‘PLU
Descontar de la 6ª Proporción’.

**Proportion7PluDiscount** **_(integer)_** : Valor entero que corresponde con el campo ‘PLU Descontar de la 7ª Proporción’.

**Proportion7PluDiscountDescription** **_(string)_** : Cadena de texto que corresponde al campo de la descripción del ‘PLU
Descontar de la 7ª Proporción’.

**Proportion8PluDiscount** **_(integer)_** : Valor entero que corresponde con el campo ‘PLU Descontar de la 8ª Proporción’.

**Proportion8PluDiscountDescription** **_(string)_** : Cadena de texto que corresponde al campo de la descripción del ‘PLU
Descontar de la 8ª Proporción’.

**Proportion9PluDiscount** **_(integer)_** : Valor entero que corresponde con el campo ‘PLU Descontar de la 9ª Proporción’.

**Proportion9PluDiscountDescription** **_(string)_** : Cadena de texto que corresponde al campo de la descripción del ‘PLU
Descontar de la 9ª Proporción’.

**ApplyDiscountsInProp** **_(boolean)_** **:** campo lógico que corresponde al campo ‘Aplicar Descuentos Generales del Artículo en
Proporciones’.

**NotAllowedAsInvitation** **_(boolean)_** **:** campo lógico que corresponde al campo ‘No permitir cómo invitación’.

**ArtCode** **_(long)_** : Valor numérico de entre 1 y 13 dígitos correspondiente al código de del artículo a ser creado. Se puede
dejar a 0 y establecer el parámetro **_AutomaticCode_** a **True** para que la aplicación suministre el número de manera
automática.

**ProfilesList(ProfilesListInfo** **_)_** : Colección que contiene una lista de los perfiles del terminal en los que se modificarán los
datos de los artículos. Es una estructura del tipo **ProfilesListInfo** y puede estar vacía. Tiene la siguiente estructura:

**Profile(** **_integer_** **)** : Campo numérico entero entre 1 y 3 dígitos, mayor que cero.


```
ProfileName( string ) : Campo de texto indicativo con el nombre del perfil. No se utiliza más que en los mensajes de error.
```
**AllProfiles(** **_boolean_** **)** : Si este campo está a **True** , entonces ignora la lista de perfiles anterior e intenta modificar los datos de los
artículos en todos ellos, si es posible.

##### JSON de solicitud

##### {

##### "AutomaticCode": true,

##### "ArticleData": {

##### "DeptCode": 1,

##### "DeptDescription": "REFRESCOS",

##### "MenuDish": false,

##### "WebArticle": true,

##### "POS_SupplementsProfileID": 0.0,

##### "SelfOrdering_CommentsProfileID": 0.0,

##### "SelfOrdering_SupplementsProfileID": 0.0,

##### "POS_MenuID": 0.0,

##### "POS_FastfoodID": 0.0,

##### "POS_PackID": 0.0,

##### "Is_Inventoriable": true,

##### "BuyTAVCode": 1,

##### "BuyTAVPer": 10.0,

##### "TAVCode": 1,

##### "TAVPer": 10.0,

##### "AuxPrinters": "",

##### "Commissionable": false,

##### "ModifiablePrice": true,

##### "DontPrintTicketValue0": true,

##### "Weight": false,

##### "DontNotifyUnitsPrice0": true,

##### "NotifyModifyPriceUnits": false,

##### "TwoForOne": false,

##### "POS_CommentsProfileID": 0.0,


##### "ErrorMessage": null,

##### "PriceConfirmation": false,

##### "FreeDescription": false,

##### "IsCombinable": false,

##### "CombinedDescription": "",

##### "CombBasePrice1": 0.0,

##### "CombBasePrice2": 0.0,

##### "CombBasePrice3": 0.0,

##### "CombBasePrice4": 0.0,

##### "CombBasePrice5": 0.0,

##### "CombAuxPrice1": 0.0,

##### "CombAuxPrice2": 0.0,

##### "CombAuxPrice3": 0.0,

##### "CombAuxPrice4": 0.0,

##### "CombAuxPrice5": 0.0,

##### "ActivateAlwaysCombined": false,

##### "MandatoryCombined": 3,

##### "CombinedAssocType": 1,

##### "CombinedDepartmentAssoc": 0,

##### "CombinedDepartmentAssocDescription": "",

##### "CombinedMaxiscreenAssoc": 0,

##### "CombinedMaxiscreenAssocDescription": "",

##### "ApplyDiscountsInComb": false,

##### "ArtDescription": "Artículo nuevo Weblink RestAPI",

##### "Price1": 2.0,

##### "Price2": 3.0,

##### "Price3": 4.0,

##### "Price4": 5.0,

##### "Price5": 6.0,

##### "Dct1": 0.0,

##### "Dct2": 0.0,


##### "Dct3": 0.0,

##### "Dct4": 0.0,

##### "Dct5": 0.0,

##### "GraphDescrip1": "PLU",

##### "GraphDescrip2": "WEBLINK",

##### "GraphDescrip3": "RESTAPI",

##### "ExtendedArtDescription": "Esta descrípción se usa en modulos como por ejemplo Autocomanda o Worpress",

##### "Proportion1Description": "",

##### "Proportion2Active": false,

##### "Proportion3Active": false,

##### "Proportion4Active": false,

##### "Proportion5Active": false,

##### "Proportion6Active": false,

##### "Proportion7Active": false,

##### "Proportion8Active": false,

##### "Proportion9Active": false,

##### "Proportion2Amount": 0.0,

##### "Proportion3Amount": 0.0,

##### "Proportion4Amount": 0.0,

##### "Proportion5Amount": 0.0,

##### "Proportion6Amount": 0.0,

##### "Proportion7Amount": 0.0,

##### "Proportion8Amount": 0.0,

##### "Proportion9Amount": 0.0,

##### "Proportion2Description": "",

##### "Proportion3Description": "",

##### "Proportion4Description": "",

##### "Proportion5Description": "",

##### "Proportion6Description": "",

##### "Proportion7Description": "",

##### "Proportion8Description": "",


##### "Proportion9Description": "",

##### "Proportion2Price1": 0.0,

##### "Proportion3Price1": 0.0,

##### "Proportion4Price1": 0.0,

##### "Proportion5Price1": 0.0,

##### "Proportion6Price1": 0.0,

##### "Proportion7Price1": 0.0,

##### "Proportion8Price1": 0.0,

##### "Proportion9Price1": 0.0,

##### "Proportion2Price2": 0.0,

##### "Proportion3Price2": 0.0,

##### "Proportion4Price2": 0.0,

##### "Proportion5Price2": 0.0,

##### "Proportion6Price2": 0.0,

##### "Proportion7Price2": 0.0,

##### "Proportion8Price2": 0.0,

##### "Proportion9Price2": 0.0,

##### "Proportion2Price3": 0.0,

##### "Proportion3Price3": 0.0,

##### "Proportion4Price3": 0.0,

##### "Proportion5Price3": 0.0,

##### "Proportion6Price3": 0.0,

##### "Proportion7Price3": 0.0,

##### "Proportion8Price3": 0.0,

##### "Proportion9Price3": 0.0,

##### "Proportion2Price4": 0.0,

##### "Proportion3Price4": 0.0,

##### "Proportion4Price4": 0.0,

##### "Proportion5Price4": 0.0,

##### "Proportion6Price4": 0.0,

##### "Proportion7Price4": 0.0,


##### "Proportion8Price4": 0.0,

##### "Proportion9Price4": 0.0,

##### "Proportion2Price5": 0.0,

##### "Proportion3Price5": 0.0,

##### "Proportion4Price5": 0.0,

##### "Proportion5Price5": 0.0,

##### "Proportion6Price5": 0.0,

##### "Proportion7Price5": 0.0,

##### "Proportion8Price5": 0.0,

##### "Proportion9Price5": 0.0,

##### "Proportion2PluDiscount": 0,

##### "Proportion2PluDiscountDescription": "",

##### "Proportion3PluDiscount": 0,

##### "Proportion3PluDiscountDescription": "",

##### "Proportion4PluDiscount": 0,

##### "Proportion4PluDiscountDescription": "",

##### "Proportion5PluDiscount": 0,

##### "Proportion5PluDiscountDescription": "",

##### "Proportion6PluDiscount": 0,

##### "Proportion6PluDiscountDescription": "",

##### "Proportion7PluDiscount": 0,

##### "Proportion7PluDiscountDescription": "",

##### "Proportion8PluDiscount": 0,

##### "Proportion8PluDiscountDescription": "",

##### "Proportion9PluDiscount": 0,

##### "Proportion9PluDiscountDescription": "",

##### "ApplyDiscountsInProp": false,

##### "ArtCode": 0

##### },

##### "ProfilesList": [

##### {


##### "Profile": 1,

##### "ProfileName": "PERFIL 1"

##### }

##### ],

##### "AllProfiles": false

##### }

##### Parámetros de salida

**ErrorMessage** **_(string)_** : Cadena que almacenará posibles errores. Si la consulta se realizó correctamente, estará vacía.

**ListaErroresArtículos** **_(string List)_** : Lista de string que contendrá los posibles errores creando artículos que se hubieran podido
producir en artículos implicados en la actualización.

##### JSON de respuesta

{
"ErrorMessage": "",
"ListaErroresArticulo": []
}


### ModifyPricesArticles

Mediante este comando se podrá modificar los PVP y Descuentos de una serie de artículos ya existentes a partir de unos datos de
entrada y, si procede, modifica dichos datos de los artículos en una serie de perfiles de terminales.

##### Ruta de llamada

/API/Articles/ModifyPrices

##### Parámetros de entrada

**ArticlesDataList(** **_UpdateArticlesListPrice_** **)** : Colección de datos de los artículos cuyos precios y descuentos se van a modificar. Se trata
de una estructura del tipo **UpdateArticlesListPrice** que contiene los siguientes campos:

```
ArtCode( long ) :Valor entero mayor que cero de hasta 13 dígitos que indica el código del artículo a modificar.
```
```
ArtDescription( string ) : Cadena de un máximo de 40 caracteres, con la descripción del artículo. La descripción es indicativa,
no se modificará.
```
```
Price1( decimal ) : Valor decimal con el PVP1.
```
```
Price2( decimal ) : Valor decimal con el PVP2.
```
```
Price3( decimal ) : Valor decimal con el PVP3.
```
```
Price4( decimal ) : Valor decimal con el PVP4.
```
```
Price5( decimal ) : Valor decimal con el PVP5.
```
```
Dct1( decimal ) : Valor decimal entre 0 y 99,99 con el porcentaje de descuento 1.
```
```
Dct2( decimal ) : Valor decimal entre 0 y 99,99 con el porcentaje de descuento 2.
```
```
Dct3( decimal ) : Valor decimal entre 0 y 99,99 con el porcentaje de descuento 3.
```
```
Dct4( decimal ) : Valor decimal entre 0 y 99,99 con el porcentaje de descuento 4.
```
```
Dct5( decimal ) : Valor decimal entre 0 y 99,99 con el porcentaje de descuento 5.
```
```
CombBasePrice1 (decimal) : Valor decimal del precio base 1 del combinado.
```
```
CombBasePrice2 (decimal) : Valor decimal del precio base 2 del combinado.
```
```
CombBasePrice3 (decimal) : Valor decimal del precio base 3 del combinado.
```
```
CombBasePrice4 (decimal) : Valor decimal del precio base 4 del combinado.
```
```
CombBasePrice5 (decimal) : Valor decimal del precio base 5 del combinado.
```
```
CombAuxPrice1 (decimal) : Valor decimal del precio auxiliar 1 del combinado.
```
```
CombAuxPrice2 (decimal) : Valor decimal del precio auxiliar 2 del combinado.
```
```
CombAuxPrice3 (decimal) : Valor decimal del precio auxiliar 3 del combinado.
```

**CombAuxPrice4** **_(decimal)_** : Valor decimal del precio auxiliar 4 del combinado.

**CombAuxPrice5** **_(decimal)_** : Valor decimal del precio auxiliar 5 del combinado.

**Proportion2Price1** **_(decimal)_** : Valor decimal con el PVP1 de la 2ª Proporción.

**Proportion3Price1** **_(decimal)_** : Valor decimal con el PVP1 de la 3ª Proporción.

**Proportion4Price1** **_(decimal)_** : Valor decimal con el PVP1 de la 4ª Proporción.

**Proportion5Price1** **_(decimal)_** : Valor decimal con el PVP1 de la 5ª Proporción.

**Proportion6Price1** **_(decimal)_** : Valor decimal con el PVP1 de la 6ª Proporción.

**Proportion7Price1** **_(decimal)_** : Valor decimal con el PVP1 de la 7ª Proporción.

**Proportion8Price1** **_(decimal)_** : Valor decimal con el PVP1 de la 8ª Proporción.

**Proportion9Price1** **_(decimal)_** : Valor decimal con el PVP1 de la 9ª Proporción.

**Proportion2Price2** **_(decimal)_** : Valor decimal con el PVP2 de la 2ª Proporción.

**Proportion3Price2** **_(decimal)_** : Valor decimal con el PVP2 de la 3ª Proporción.

**Proportion4Price2** **_(decimal)_** : Valor decimal con el PVP2 de la 4ª Proporción.

**Proportion5Price2** **_(decimal)_** : Valor decimal con el PVP2 de la 5ª Proporción.

**Proportion6Price2** **_(decimal)_** : Valor decimal con el PVP2 de la 6ª Proporción.

**Proportion7Price2** **_(decimal)_** : Valor decimal con el PVP2 de la 7ª Proporción.

**Proportion8Price2** **_(decimal)_** : Valor decimal con el PVP2 de la 8ª Proporción.

**Proportion9Price2** **_(decimal)_** : Valor decimal con el PVP2 de la 9ª Proporción.

**Proportion2Price3** **_(decimal)_** : Valor decimal con el PVP3 de la 2ª Proporción.

**Proportion3Price3** **_(decimal)_** : Valor decimal con el PVP3 de la 3ª Proporción.

**Proportion4Price3** **_(decimal)_** : Valor decimal con el PVP3 de la 4ª Proporción.

**Proportion5Price3** **_(decimal)_** : Valor decimal con el PVP3 de la 5ª Proporción.

**Proportion6Price3** **_(decimal)_** : Valor decimal con el PVP3 de la 6ª Proporción.

**Proportion7Price3** **_(decimal)_** : Valor decimal con el PVP3 de la 7ª Proporción.

**Proportion8Price3** **_(decimal)_** : Valor decimal con el PVP3 de la 8ª Proporción.

**Proportion9Price3** **_(decimal)_** : Valor decimal con el PVP3 de la 9ª Proporción.

**Proportion2Price4** **_(decimal)_** : Valor decimal con el PVP4 de la 2ª Proporción.

**Proportion3Price4** **_(decimal)_** : Valor decimal con el PVP4 de la 3ª Proporción.

**Proportion4Price4** **_(decimal)_** : Valor decimal con el PVP4 de la 4ª Proporción.


```
Proportion5Price4 (decimal) : Valor decimal con el PVP4 de la 5ª Proporción.
```
```
Proportion6Price4 (decimal) : Valor decimal con el PVP4 de la 6ª Proporción.
```
```
Proportion7Price4 (decimal) : Valor decimal con el PVP4 de la 7ª Proporción.
```
```
Proportion8Price4 (decimal) : Valor decimal con el PVP4 de la 8ª Proporción.
```
```
Proportion9Price4 (decimal) : Valor decimal con el PVP4 de la 9ª Proporción.
```
```
Proportion2Price5 (decimal) : Valor decimal con el PVP5 de la 2ª Proporción.
```
```
Proportion3Price5 (decimal) : Valor decimal con el PVP5 de la 3ª Proporción.
```
```
Proportion4Price5 (decimal) : Valor decimal con el PVP5 de la 4ª Proporción.
```
```
Proportion5Price5 (decimal) : Valor decimal con el PVP5 de la 5ª Proporción.
```
```
Proportion6Price5 (decimal) : Valor decimal con el PVP5 de la 6ª Proporción.
```
```
Proportion7Price5 (decimal) : Valor decimal con el PVP5 de la 7ª Proporción.
```
```
Proportion8Price5 (decimal) : Valor decimal con el PVP5 de la 8ª Proporción.
```
```
Proportion9Price5 (decimal) : Valor decimal con el PVP5 de la 9ª Proporción.
```
```
TyC_SalesPrice1 : Valor decimal correspondiente al campo ‘Precio Rebajas 1” en la aplicación de Talla y Color.
```
```
TyC_SalesPrice2 : Valor decimal correspondiente al campo ‘Precio Rebajas 2” en la aplicación de Talla y Color.
```
```
TyC_SalesPrice3 : Valor decimal correspondiente al campo ‘Precio Rebajas 3” en la aplicación de Talla y Color.
```
```
TyC_DtoSalesPrice1 : Valor decimal correspondiente al campo ‘Descuento Rebajas 1” en la aplicación de Talla y Color.
```
```
TyC_DtoSalesPrice2 : Valor decimal correspondiente al campo ‘Descuento Rebajas 2” en la aplicación de Talla y Color.
```
```
TyC_DtoSalesPrice3 : Valor decimal correspondiente al campo ‘Descuento Rebajas 3” en la aplicación de Talla y Color.
```
**ProfilesList(ProfilesListInfo** **_)_** : Colección que contiene una lista de los perfiles del terminal en los que se modificarán los datos de los
artículos. Es una estructura del tipo **ProfilesListInfo** y puede estar vacía. Tiene la siguiente estructura:

```
Profile( integer ) : Campo numérico entero entre 1 y 3 dígitos, mayor que cero.
```
```
ProfileName( string ) : Campo de texto indicativo con el nombre del perfil. No se utiliza más que en los mensajes de error.
```
**AllProfiles(** **_boolean_** **)** : si este campo está a **True** , entonces ignora la lista de perfiles anterior e intenta modificar los datos de los
artículos en todos ellos, si es posible.

##### JSON de solicitud

##### {

##### "ArticlesData": {

##### "ArtCode": 1001,

##### "ArtDescription": "Coca - Cola",


##### "Price1": 1.05,

##### "Price2": 2.05,

##### "Price3": 3.05,

##### "Price4": 4.05,

##### "Price5": 5.05,

##### "Dct1": 0.0,

##### "Dct2": 0.0,

##### "Dct3": 0.0,

##### "Dct4": 0.0,

##### "Dct5": 0.0,

##### "CombBasePrice1": 0.0,

##### "CombBasePrice2": 0.0,

##### "CombBasePrice3": 0.0,

##### "CombBasePrice4": 0.0,

##### "CombBasePrice5": 0.0,

##### "CombAuxPrice1": 0.0,

##### "CombAuxPrice2": 0.0,

##### "CombAuxPrice3": 0.0,

##### "CombAuxPrice4": 0.0,

##### "CombAuxPrice5": 0.0,

##### "Proportion2Price1": 0.0,

##### "Proportion2Price2": 0.0,

##### "Proportion2Price3": 0.0,

##### "Proportion2Price4": 0.0,

##### "Proportion2Price5": 0.0,

##### "Proportion3Price1": 0.0,

##### "Proportion3Price2": 0.0,

##### "Proportion3Price3": 0.0,

##### "Proportion3Price4": 0.0,

##### "Proportion3Price5": 0.0,

##### "Proportion4Price1": 0.0,


##### "Proportion4Price2": 0.0,

##### "Proportion4Price3": 0.0,

##### "Proportion4Price4": 0.0,

##### "Proportion4Price5": 0.0,

##### "Proportion5Price1": 0.0,

##### "Proportion5Price2": 0.0,

##### "Proportion5Price3": 0.0,

##### "Proportion5Price4": 0.0,

##### "Proportion5Price5": 0.0,

##### "Proportion6Price1": 0.0,

##### "Proportion6Price2": 0.0,

##### "Proportion6Price3": 0.0,

##### "Proportion6Price4": 0.0,

##### "Proportion6Price5": 0.0,

##### "Proportion7Price1": 0.0,

##### "Proportion7Price2": 0.0,

##### "Proportion7Price3": 0.0,

##### "Proportion7Price4": 0.0,

##### "Proportion7Price5": 0.0,

##### "Proportion8Price1": 0.0,

##### "Proportion8Price2": 0.0,

##### "Proportion8Price3": 0.0,

##### "Proportion8Price4": 0.0,

##### "Proportion8Price5": 0.0,

##### "Proportion9Price1": 0.0,

##### "Proportion9Price2": 0.0,

##### "Proportion9Price3": 0.0,

##### "Proportion9Price4": 0.0,

##### "Proportion9Price5": 0.0,

##### "TyC_SalesPrice1": 0.0,

##### "TyC_SalesPrice2": 0.0,


##### "TyC_SalesPrice3": 0.0,

##### "TyC_DtoSalesPrice1": 0.0,

##### "TyC_DtoSalesPrice2": 0.0,

##### "TyC_DtoSalesPrice3": 0.0

##### },

##### "ProfilesList": [

##### {

##### "Profile": 1,

##### "ProfileName": "PERFIL 1"

##### }

##### ],

##### "AllProfiles": false

##### }

##### Parámetros de salida

**ErrorMessage** **_(string)_** : Cadena que almacenará posibles errores. Si la consulta se realizó correctamente, estará vacía.

**ListaErroresArtículos** **_(string List)_** : Lista de string que contendrá los posibles errores actualizando precios y descuentos que se
hubieran podido producir en artículos implicados en la actualización.

##### JSON de respuesta

{
"ErrorMessage": "",
"ListaErroresArticulo": []
}


### ModifyArticleAndUpdateProfile

Mediante este comando se podrá modifica un artículo ya existente a partir de unos datos de entrada y, si procede, modifica los
datos del artículo en una serie de perfiles de terminales.

##### Ruta de llamada

/API/Articles/ModifyAndUpdateProfiles

##### Parámetros de entrada

**ArticleData(** **_ArticleListDataType_** **)** , Colección de objetos **_ArticleListDataType_** con la información para la creación del artículo. Tiene
una estructura del tipo **ArticleListDataTye** :

```
DeptCode (integer) : Valor con el código de departamento de venta asociado al artículo.
```
```
DeptDescription (string) : Cadena con la descripción del departamento de venta asociado al artículo.
```
```
MenuDish (boolean) : Campo lógico que corresponde con el campo ‘Plato de Menú’ del artículo.
```
```
WebArticle (boolean) : Campo lógico que corresponde con el campo ‘Artículo Web’ del artículo.
```
```
POS_SupplementsProfileID (decimal) : Valor con el código de perfil de suplementos asociado al artículo.
```
```
SelfOrdering_CommentsProfileID (decimal) : Valor con el código de perfil de comentarios asociado a la configuración de
autocomanda del artículo.
```
```
SelfOrdering_SupplementsProfileID (decimal) : Valor con el código de perfil de suplementos asociado a la configuración de
autocomanda del artículo.
```
```
POS_MenuID (decimal) : Valor con el código de definición de menú asociado al artículo.
```
```
POS_FastfoodID (decimal) : Valor con el código de definición fastfood asociado al artículo.
```
```
POS_PackID (decimal) : Valor con el código de definición de pack asociado al artículo.
```
```
Is_Inventoriable (boolean) : Campo lógico que corresponde con el campo ‘Inventariable’ del artículo.
```
```
BuyTAVCode (integer) : Valor con el código de IVA de compra asociado al artículo.
```
```
BuyTAVPer (decimal) : Valor decimal con el porcentaje del IVA de compra asociado al artículo.
```
```
TAVCode (integer) : Valor con el código de IVA de venta asociado al artículo.
```
```
TAVPer (decimal) : Valor decimal con el porcentaje del IVA de venta asociado al artículo.
```
```
AuxPrinters (string) : Cadena con la secuencia de impresoras auxiliares asociadas al artículo.
```
```
Commissionable (boolean) : Campo lógico que corresponde con el campo ‘Artículo Comisionable’ del artículo.
```
```
ModifiablePrice (boolean) : Campo lógico que corresponde con el campo ‘Precio Modificable’ del artículo.
```
```
DontPrintTicketValue0 (boolean) : Campo lógico que corresponde con el campo ‘No Imprimir en Factura si Valor 0’ del
artículo.
```

**Weight** **_(boolean)_** : Campo lógico que corresponde con el campo ‘Artículo de Peso’ del artículo.

**DontNotifyUnitsPrice0** **_(boolean)_** : Campo lógico que corresponde con el campo ‘No Avisar si unidades o Precio es 0’ del
artículo.

**NotifyModifyPriceUnits** **_(boolean)_** : Campo lógico que corresponde con el campo ‘Avisar para Modificar Precio y/o
Unidades’ del artículo.

**TwoForOne** **_(boolean)_** : Campo lógico que corresponde con el campo ‘Artículo Dos x Uno” del artículo.

**POS_CommentsProfileID (decimal)** : Valor con el código de perfil de comentarios asociado al artículo.

**ErrorMessage** **_(string)_** : Cadena que almacenará posibles errores. Si la consulta se realizó correctamente, estará vacía.

**PriceConfirmation** **_(boolean)_** : Campo lógico que corresponde con el campo ‘Confirmación Precio’ del artículo.

**FreeDescription** **_(boolean)_** : Campo lógico que corresponde con el campo ‘Descripción Libre’ del artículo.

**IsCombinable** **_(boolean)_** : Campo lógico que indica si el artículo es combinable.

**CombinedDescription** **_(string)_** : Cadena de texto que corresponde a la descripción del combinado (máx. 15 caracteres).

**CombBasePrice1** **_(decimal)_** : Valor decimal del precio base 1 del combinado.

**CombBasePrice2** **_(decimal)_** : Valor decimal del precio base 2 del combinado.

**CombBasePrice3** **_(decimal)_** : Valor decimal del precio base 3 del combinado.

**CombBasePrice4** **_(decimal)_** : Valor decimal del precio base 4 del combinado.

**CombBasePrice5** **_(decimal)_** : Valor decimal del precio base 5 del combinado.

**CombAuxPrice1** **_(decimal)_** : Valor decimal del precio auxiliar 1 del combinado.

**CombAuxPrice2** **_(decimal)_** : Valor decimal del precio auxiliar 2 del combinado.

**CombAuxPrice3** **_(decimal)_** : Valor decimal del precio auxiliar 3 del combinado.

**CombAuxPrice4** **_(decimal)_** : Valor decimal del precio auxiliar 4 del combinado.

**CombAuxPrice5** **_(decimal)_** : Valor decimal del precio auxiliar 5 del combinado.

**ActivateAlwaysCombined** **_(boolean)_** : Campo lógico que corresponde al campo ‘Activar Siempre Combinado’.

**MandatoryCombined** **_(integer)_** : Valor entero que indica la Obligatoriedad de Combinado: 1: Obligatoriedad de Base; 2:
Obligatoriedad de Auxiliar; 3: Indiferente.

**CombinedAssocType** **_(integer)_** : Valor entero que corresponde al campo ‘Departamentos Asociados al Combinado’: 1: No
Asociar; 2: Asociar a un Departamento; 3: Asociar a una Maxi-Pantalla.

**CombinedDepartmentAssoc** **_(integer)_** : Valor entero que indica el código de Departamento Asociado a los combinados.

**CombinedDepartmentAssocDescription** **_(string)_** : Cadena de texto que corresponde a la descripción del Departamento
Asociado a los combinados (máx. 40 caracteres).

**CombinedMaxiscreenAssoc** **_(integer)_** : Valor entero que indica el código de Maxipantalla Asociada a los combinados.


**CombinedMaxiscreenAssocDescription** **_(string)_** : Cadena de texto que corresponde a la descripción de la Maxipantalla
Asociada a los combinados (máx. 40 caracteres).

**ApplyDiscountsInComb** **_(boolean)_** : Campo lógico que corresponde al campo ‘Aplicar Descuentos Generales del Artículo en
Combinados’.

**ArtDescription** **_(string)_** : Cadena de texto que corresponde a la descripción del artículo (máx. 40 caracteres).

**Price1** **_(decimal)_** : Valor decimal con el PVP1 del artículo.

**Price2** **_(decimal)_** : Valor decimal con el PVP2 del artículo.

**Price3** **_(decimal)_** : Valor decimal con el PVP3 del artículo.

**Price4** **_(decimal)_** : Valor decimal con el PVP4 del artículo.

**Price5** **_(decimal)_** : Valor decimal con el PVP5 del artículo.

**Dct1** **_(decimal)_** : Valor decimal con el porcentaje de descuento 1 del artículo.

**Dct2** **_(decimal)_** : Valor decimal con el porcentaje de descuento 2 del artículo.

**Dct3** **_(decimal)_** : Valor decimal con el porcentaje de descuento 3 del artículo.

**Dct4** **_(decimal)_** : Valor decimal con el porcentaje de descuento 4 del artículo.

**Dct5** **_(decimal)_** : Valor decimal con el porcentaje de descuento 5 del artículo.

**GraphDescrip1** **_(string)_** : Cadena de texto que corresponde a la Descripción Gráfica de la Línea 1 del artículo.

**GraphDescrip2** **_(string)_** : Cadena de texto que corresponde a la Descripción Gráfica de la Línea 2 del artículo.

**GraphDescrip3** **_(string)_** : Cadena de texto que corresponde a la Descripción Gráfica de la Línea 3 del artículo.

**ExtendedArtDescription** **_(string)_** : Cadena de texto que corresponde a la Descripción Extendida del artículo.

**Proportion1Description** **_(string)_** : Cadena de texto que corresponde al campo ‘Descripción de la 1ª Proporción’ (máx. 40
caracteres).

**Proportion2Active** **_(boolean)_** : Campo lógico que corresponde al campo ‘Activar 2ª Proporción’.

**Proportion3Active** **_(boolean)_** : Campo lógico que corresponde al campo ‘Activar 3ª Proporción’.

**Proportion4Active** **_(boolean)_** : Campo lógico que corresponde al campo ‘Activar 4ª Proporción’.

**Proportion5Active** **_(boolean)_** : Campo lógico que corresponde al campo ‘Activar 5ª Proporción’.

**Proportion6Active** **_(boolean)_** : Campo lógico que corresponde al campo ‘Activar 6ª Proporción’.

**Proportion7Active** **_(boolean)_** : Campo lógico que corresponde al campo ‘Activar 7ª Proporción’.

**Proportion8Active** **_(boolean)_** : Campo lógico que corresponde al campo ‘Activar 8ª Proporción’.

**Proportion9Active** **_(boolean)_** : Campo lógico que corresponde al campo ‘Activar 9ª Proporción’.

**Proportion2Amount** **_(decimal)_** : Valor decimal correspondiente al campo ‘Cantidad 2ª Proporción’.


**Proportion3Amount** **_(decimal)_** : Valor decimal correspondiente al campo ‘Cantidad 3ª Proporción’.

**Proportion4Amount** **_(decimal)_** : Valor decimal correspondiente al campo ‘Cantidad 4ª Proporción’.

**Proportion5Amount** **_(decimal)_** : Valor decimal correspondiente al campo ‘Cantidad 5ª Proporción’.

**Proportion6Amount** **_(decimal)_** : Valor decimal correspondiente al campo ‘Cantidad 6ª Proporción’.

**Proportion7Amount** **_(decimal)_** : Valor decimal correspondiente al campo ‘Cantidad 7ª Proporción’.

**Proportion8Amount** **_(decimal)_** : Valor decimal correspondiente al campo ‘Cantidad 8ª Proporción’.

**Proportion9Amount** **_(decimal)_** : Valor decimal correspondiente al campo ‘Cantidad 9ª Proporción’.

**Proportion2Description** **_(string)_** : Cadena de texto que corresponde al campo ‘Descripción de la 2ª Proporción’ (máx. 40
caracteres).

**Proportion3Description** **_(string)_** : Cadena de texto que corresponde al campo ‘Descripción de la 3ª Proporción’ (máx. 40
caracteres).

**Proportion4Description** **_(string)_** : Cadena de texto que corresponde al campo ‘Descripción de la 4ª Proporción’ (máx. 40
caracteres).

**Proportion5Description** **_(string)_** : Cadena de texto que corresponde al campo ‘Descripción de la 5ª Proporción’ (máx. 40
caracteres).

**Proportion6Description** **_(string)_** : Cadena de texto que corresponde al campo ‘Descripción de la 6ª Proporción’ (máx. 40
caracteres).

**Proportion7Description** **_(string)_** : Cadena de texto que corresponde al campo ‘Descripción de la 7ª Proporción’ (máx. 40
caracteres).

**Proportion8Description** **_(string)_** : Cadena de texto que corresponde al campo ‘Descripción de la 8ª Proporción’ (máx. 40
caracteres).

##### Proportion9Description (string) : Cadena de texto que corresponde al campo ‘Descripción de la 9ª Proporción’

##### (máx. 40 caracteres).

**Proportion2Price1** **_(decimal)_** : Valor decimal con el PVP1 de la 2ª Proporción.

**Proportion3Price1** **_(decimal)_** : Valor decimal con el PVP1 de la 3ª Proporción.

**Proportion4Price1** **_(decimal)_** : Valor decimal con el PVP1 de la 4ª Proporción.

**Proportion5Price1** **_(decimal)_** : Valor decimal con el PVP1 de la 5ª Proporción.

**Proportion6Price1** **_(decimal)_** : Valor decimal con el PVP1 de la 6ª Proporción.

**Proportion7Price1** **_(decimal)_** : Valor decimal con el PVP1 de la 7ª Proporción.

**Proportion8Price1** **_(decimal)_** : Valor decimal con el PVP1 de la 8ª Proporción.

**Proportion9Price1** **_(decimal)_** : Valor decimal con el PVP1 de la 9ª Proporción.

**Proportion2Price2** **_(decimal)_** : Valor decimal con el PVP2 de la 2ª Proporción.

**Proportion3Price2** **_(decimal)_** : Valor decimal con el PVP2 de la 3ª Proporción.


**Proportion4Price2** **_(decimal)_** : Valor decimal con el PVP2 de la 4ª Proporción.

**Proportion5Price2** **_(decimal)_** : Valor decimal con el PVP2 de la 5ª Proporción.

**Proportion6Price2** **_(decimal)_** : Valor decimal con el PVP2 de la 6ª Proporción.

**Proportion7Price2** **_(decimal)_** : Valor decimal con el PVP2 de la 7ª Proporción.

**Proportion8Price2** **_(decimal)_** : Valor decimal con el PVP2 de la 8ª Proporción.

**Proportion9Price2** **_(decimal)_** : Valor decimal con el PVP2 de la 9ª Proporción.

**Proportion2Price3** **_(decimal)_** : Valor decimal con el PVP3 de la 2ª Proporción.

**Proportion3Price3** **_(decimal)_** : Valor decimal con el PVP3 de la 3ª Proporción.

**Proportion4Price3** **_(decimal)_** : Valor decimal con el PVP3 de la 4ª Proporción.

**Proportion5Price3** **_(decimal)_** : Valor decimal con el PVP3 de la 5ª Proporción.

**Proportion6Price3** **_(decimal)_** : Valor decimal con el PVP3 de la 6ª Proporción.

**Proportion7Price3** **_(decimal)_** : Valor decimal con el PVP3 de la 7ª Proporción.

**Proportion8Price3** **_(decimal)_** : Valor decimal con el PVP3 de la 8ª Proporción.

**Proportion9Price3** **_(decimal)_** : Valor decimal con el PVP3 de la 9ª Proporción.

**Proportion2Price4** **_(decimal)_** : Valor decimal con el PVP4 de la 2ª Proporción.

**Proportion3Price4** **_(decimal)_** : Valor decimal con el PVP4 de la 3ª Proporción.

**Proportion4Price4** **_(decimal)_** : Valor decimal con el PVP4 de la 4ª Proporción.

**Proportion5Price4** **_(decimal)_** : Valor decimal con el PVP4 de la 5ª Proporción.

**Proportion6Price4** **_(decimal)_** : Valor decimal con el PVP4 de la 6ª Proporción.

**Proportion7Price4** **_(decimal)_** : Valor decimal con el PVP4 de la 7ª Proporción.

**Proportion8Price4** **_(decimal)_** : Valor decimal con el PVP4 de la 8ª Proporción.

**Proportion9Price4** **_(decimal)_** : Valor decimal con el PVP4 de la 9ª Proporción.

**Proportion2Price5** **_(decimal)_** : Valor decimal con el PVP5 de la 2ª Proporción.

**Proportion3Price5** **_(decimal)_** : Valor decimal con el PVP5 de la 3ª Proporción.

**Proportion4Price5** **_(decimal)_** : Valor decimal con el PVP5 de la 4ª Proporción.

**Proportion5Price5** **_(decimal)_** : Valor decimal con el PVP5 de la 5ª Proporción.

**Proportion6Price5** **_(decimal)_** : Valor decimal con el PVP5 de la 6ª Proporción.

**Proportion7Price5** **_(decimal)_** : Valor decimal con el PVP5 de la 7ª Proporción.

**Proportion8Price5** **_(decimal)_** : Valor decimal con el PVP5 de la 8ª Proporción.


**Proportion9Price5** **_(decimal)_** : Valor decimal con el PVP5 de la 9ª Proporción.

**Proportion2PluDiscount** **_(integer)_** : Valor entero que corresponde con el campo ‘PLU Descontar de la 2ª Proporción’.

**Proportion2PluDiscountDescription** **_(string)_** : Cadena de texto que corresponde al campo de la descripción del ‘PLU
Descontar de la 2ª Proporción’.

**Proportion3PluDiscount** **_(integer)_** : Valor entero que corresponde con el campo ‘PLU Descontar de la 3ª Proporción’.

**Proportion3PluDiscountDescription** **_(string)_** : Cadena de texto que corresponde al campo de la descripción del ‘PLU
Descontar de la 3ª Proporción’.

**Proportion4PluDiscount** **_(integer)_** : Valor entero que corresponde con el campo ‘PLU Descontar de la 4ª Proporción’.

**Proportion4PluDiscountDescription** **_(string)_** : Cadena de texto que corresponde al campo de la descripción del ‘PLU
Descontar de la 4ª Proporción’.

**Proportion5PluDiscount** **_(integer)_** : Valor entero que corresponde con el campo ‘PLU Descontar de la 5ª Proporción’.

**Proportion5PluDiscountDescription** **_(string)_** : Cadena de texto que corresponde al campo de la descripción del ‘PLU
Descontar de la 5ª Proporción’.

**Proportion6PluDiscount** **_(integer)_** : Valor entero que corresponde con el campo ‘PLU Descontar de la 6ª Proporción’.

**Proportion6PluDiscountDescription** **_(string)_** : Cadena de texto que corresponde al campo de la descripción del ‘PLU
Descontar de la 6ª Proporción’.

**Proportion7PluDiscount** **_(integer)_** : Valor entero que corresponde con el campo ‘PLU Descontar de la 7ª Proporción’.

**Proportion7PluDiscountDescription** **_(string)_** : Cadena de texto que corresponde al campo de la descripción del ‘PLU
Descontar de la 7ª Proporción’.

**Proportion8PluDiscount** **_(integer)_** : Valor entero que corresponde con el campo ‘PLU Descontar de la 8ª Proporción’.

**Proportion8PluDiscountDescription** **_(string)_** : Cadena de texto que corresponde al campo de la descripción del ‘PLU
Descontar de la 8ª Proporción’.

**Proportion9PluDiscount** **_(integer)_** : Valor entero que corresponde con el campo ‘PLU Descontar de la 9ª Proporción’.

**Proportion9PluDiscountDescription** **_(string)_** : Cadena de texto que corresponde al campo de la descripción del ‘PLU
Descontar de la 9ª Proporción’.

**ApplyDiscountsInProp** **_(boolean)_** **:** campo lógico que corresponde al campo ‘Aplicar Descuentos Generales del Artículo en
Proporciones’.

**NotAllowedAsInvitation** **_(boolean)_** **:** campo lógico que corresponde al campo ‘No permitir cómo invitación’.

**ArtCode** **_(long)_** : Valor numérico de entre 1 y 13 dígitos correspondiente al código de del artículo a ser modificado.

**ProfilesList(ProfilesListInfo** **_)_** : Colección que contiene una lista de los perfiles del terminal en los que se modificarán los
datos de los artículos. Es una estructura del tipo **ProfilesListInfo** y puede estar vacía. Tiene la siguiente estructura:

**Profile(** **_integer_** **)** : Campo numérico entero entre 1 y 3 dígitos, mayor que cero.

**ProfileName(** **_string_** **)** : Campo de texto indicativo con el nombre del perfil. No se utiliza más que en los mensajes de error.


**AllProfiles(** **_boolean_** **)** : Si este campo está a **True** , entonces ignora la lista de perfiles anterior e intenta modificar los datos de los
artículos en todos ellos, si es posible.

##### JSON de solicitud

{
"ArticleData": {
"DeptCode": 1,
"DeptDescription": "REFRESCOS",
"MenuDish": false,
"WebArticle": true,
"POS_SupplementsProfileID": 0.0,
"SelfOrdering_CommentsProfileID": 0.0,
"SelfOrdering_SupplementsProfileID": 0.0,
"POS_MenuID": 0.0,
"POS_FastfoodID": 0.0,
"POS_PackID": 0.0,
"Is_Inventoriable": true,
"BuyTAVCode": 1,
"BuyTAVPer": 10.0,
"TAVCode": 1,
"TAVPer": 10.0,
"AuxPrinters": "",
"Commissionable": false,
"ModifiablePrice": true,
"DontPrintTicketValue0": true,
"Weight": false,
"DontNotifyUnitsPrice0": true,
"NotifyModifyPriceUnits": false,
"TwoForOne": false,
"POS_CommentsProfileID": 0.0,
"ErrorMessage": null,
"PriceConfirmation": false,
"FreeDescription": false,
"IsCombinable": false,
"CombinedDescription": "",
"CombBasePrice1": 0.0,
"CombBasePrice2": 0.0,
"CombBasePrice3": 0.0,
"CombBasePrice4": 0.0,
"CombBasePrice5": 0.0,
"CombAuxPrice1": 0.0,
"CombAuxPrice2": 0.0,
"CombAuxPrice3": 0.0,
"CombAuxPrice4": 0.0,
"CombAuxPrice5": 0.0,
"ActivateAlwaysCombined": false,
"MandatoryCombined": 3,
"CombinedAssocType": 1,
"CombinedDepartmentAssoc": 0,
"CombinedDepartmentAssocDescription": "",
"CombinedMaxiscreenAssoc": 0,
"CombinedMaxiscreenAssocDescription": "",
"ApplyDiscountsInComb": false,
"ArtDescription": "ARTÍCULO DE BROMA",
"Price1": 2.0,
"Price2": 3.0,
"Price3": 4.0,
"Price4": 5.0,
"Price5": 6.0,
"Dct1": 0.0,
"Dct2": 0.0,
"Dct3": 0.0,
"Dct4": 0.0,


"Dct5": 0.0,
"GraphDescrip1": "PLU",
"GraphDescrip2": "WEBLINK",
"GraphDescrip3": "RESTAPI",
"ExtendedArtDescription": "Descripción extendida modificada desde WeblinkRestAPI",
"Proportion1Description": "",
"Proportion2Active": false,
"Proportion3Active": false,
"Proportion4Active": false,
"Proportion5Active": false,
"Proportion6Active": false,
"Proportion7Active": false,
"Proportion8Active": false,
"Proportion9Active": false,
"Proportion2Amount": 0.0,
"Proportion3Amount": 0.0,
"Proportion4Amount": 0.0,
"Proportion5Amount": 0.0,
"Proportion6Amount": 0.0,
"Proportion7Amount": 0.0,
"Proportion8Amount": 0.0,
"Proportion9Amount": 0.0,
"Proportion2Description": "",
"Proportion3Description": "",
"Proportion4Description": "",
"Proportion5Description": "",
"Proportion6Description": "",
"Proportion7Description": "",
"Proportion8Description": "",
"Proportion9Description": "",
"Proportion2Price1": 0.0,
"Proportion3Price1": 0.0,
"Proportion4Price1": 0.0,
"Proportion5Price1": 0.0,
"Proportion6Price1": 0.0,
"Proportion7Price1": 0.0,
"Proportion8Price1": 0.0,
"Proportion9Price1": 0.0,
"Proportion2Price2": 0.0,
"Proportion3Price2": 0.0,
"Proportion4Price2": 0.0,
"Proportion5Price2": 0.0,
"Proportion6Price2": 0.0,
"Proportion7Price2": 0.0,
"Proportion8Price2": 0.0,
"Proportion9Price2": 0.0,
"Proportion2Price3": 0.0,
"Proportion3Price3": 0.0,
"Proportion4Price3": 0.0,
"Proportion5Price3": 0.0,
"Proportion6Price3": 0.0,
"Proportion7Price3": 0.0,
"Proportion8Price3": 0.0,
"Proportion9Price3": 0.0,
"Proportion2Price4": 0.0,
"Proportion3Price4": 0.0,
"Proportion4Price4": 0.0,
"Proportion5Price4": 0.0,
"Proportion6Price4": 0.0,
"Proportion7Price4": 0.0,
"Proportion8Price4": 0.0,
"Proportion9Price4": 0.0,
"Proportion2Price5": 0.0,
"Proportion3Price5": 0.0,
"Proportion4Price5": 0.0,
"Proportion5Price5": 0.0,


"Proportion6Price5": 0.0,
"Proportion7Price5": 0.0,
"Proportion8Price5": 0.0,
"Proportion9Price5": 0.0,
"Proportion2PluDiscount": 0,
"Proportion2PluDiscountDescription": "",
"Proportion3PluDiscount": 0,
"Proportion3PluDiscountDescription": "",
"Proportion4PluDiscount": 0,
"Proportion4PluDiscountDescription": "",
"Proportion5PluDiscount": 0,
"Proportion5PluDiscountDescription": "",
"Proportion6PluDiscount": 0,
"Proportion6PluDiscountDescription": "",
"Proportion7PluDiscount": 0,
"Proportion7PluDiscountDescription": "",
"Proportion8PluDiscount": 0,
"Proportion8PluDiscountDescription": "",
"Proportion9PluDiscount": 0,
"Proportion9PluDiscountDescription": "",
"ApplyDiscountsInProp": false,
"ArtCode": 1
},
"ProfilesList": [
{
"Profile": 1,
"ProfileName": "PERFIL 1"
}
],
"AllProfiles": false
}

##### Parámetros de salida

**ErrorMessage** ( **string** ): Cadena de errores. Si todo ha ido bien, estará vacía. Contiene errores generales de validación.

**ListaErroresArticulo** ( **of string** ): Lista de string que contendrá los posibles errores modificando artículos que se hubieran podido
producir en artículos implicados en la actualización.

##### JSON de respuesta

{
"ErrorMessage": "",
"ListaErroresArticulo": []
}


## Categoría Clientes

En esta categoría se incluirán todos los comandos que permitirán interactuar con datos de clientes. Los comandos disponibles para
esta categoría serán los siguientes:

1. **ExportCustomers** : Obtiene una lista de clientes.
2. **CreateCustomer** : Crea un cliente en la base de datos.


### ExportCustomers

Exporta los clientes contenidos para el intervalo de clientes en el intervalo seleccionado a través de los parámetros de entrada. Se
incluirán todos los clientes con códigos contenidos en el rango especificado.

##### Ruta de llamada

/API/Customers/Export

##### Parámetros de entrada

**Customer1** **_(integer)_** : número entero de 1 a 6 dígitos mayor que cero. Corresponde al código inicial del rango de clientes a exportar.

**Customer2** **_(integer)_** : número entero de 1 a 6 dígitos mayor que cero. Corresponde al código final del rango de clientes a exportar.

##### JSON de solicitud

{
"Customer1": 1,
"Customer2": 999999
}

##### Parámetros de salida

**Customers** **_List(Customers)_** : Lista de objetos del tipo **Customers** que contendrá los datos de clientes. Cada objeto **Customers**
corresponderá a los datos de un cliente y estará compuesto por los siguientes elementos:

```
Customer (integer) : valor numérico que corresponde al código del cliente.
```
```
FiscalName (string) : Cadena de texto que corresponde al nombre fiscal del cliente.
```
```
CommercialName (string) : Cadena de texto que corresponde al nombre comercial del cliente.
```
```
Address (string) : Cadena de texto que corresponde a la dirección postal del cliente.
```
```
PostCode (string) : Cadena de texto que corresponde al código postal del cliente.
```
```
Town (string) : Cadena de texto que corresponde a la población del cliente.
```
```
Province (string) : Cadena de texto que corresponde a la provincia del cliente.
```
```
FINType(): Campo de tipo numérico que mostrará el tipo de documento de identificación del cliente. Los tipos de
documentos equivalentes a los valores son:
```
```
1 = N.I.F.
2 = N.I.F. Extranjero
3 = Pasaporte
4 = ID en País de Residencia
5 = Certificado Residencia
6 = Otro Documento
```
```
FIN (string) : cadena de texto que corresponde al número de identificación fiscal del cliente.
```
```
Landline (string) : Cadena de texto que corresponde al teléfono fijo del cliente.
```

```
MobilePhone (string) : cadena de texto que corresponde al teléfono móvil del cliente.
```
```
EMail (string) : Cadena de texto que corresponde al E-mail del cliente.
```
```
CustomerType (integer) : Valor entero que muestra la codificación del tipo de cliente.
```
```
PriceType (integer) : Valor entero que muestra el nivel de precio (de los 5 disponibles en la aplicación) que tiene asignado
el cliente.
```
```
TotalPoints (decimal) : Valor decimal que corresponde a los puntos totales del sistema de fidelización y pintos que tiene el
cliente.
```
**ErrorMessage** **_(string)_** : Cadena que almacenará posibles errores. Si la consulta se realizó correctamente, estará vacía.

##### JSON de respuesta

{
"Customers": [
{
"Customer": 1,
"FiscalName": "MANOLO GARCIA FERNANDEZ",
"CommercialName": "MANOLO GARCIA FERNANDEZ",
"Address": "C/ MAYOR 345",
"PostCode": "17520",
"Town": "PUIGCERDÀ",
"Province": "GIRONA",
"FINType": 1.0,
"FIN": "4055566625",
"Landline": "972 880881",
"MobilePhone": "",
"EMail": "",
"CustomerType": 0,
"PriceType": 1,
"TotalPoints": 0.0
},...
{
"Customer": 33,
"FiscalName": "CLIENTE DE GESTIÓN",
"CommercialName": "CLI. GEST.",
"Address": "C/ MAYOR, 21",
"PostCode": "17520",
"Town": "PUIGCERDA",
"Province": "GIRONA",
"FINType": 1.0,
"FIN": "11223344-Y",
"Landline": "972 882 000",
"MobilePhone": "609 33 66 55",
"EMail": "cliauto@cliauto.com",
"CustomerType": 0,
"PriceType": 1,
"TotalPoints": 0.0
}
],
"ErrorMessage": ""
}


### CreateCustomer

Mediante esta opción se ofrece la posibilidad de dar de alta un cliente en la base de datos suministrando los datos necsarios. Si ya
existiese el cliente, si **Overwrite** =True, sobrescribe el cliente; en caso contrario, no lo hace y emite un error informando de la
existencia del cliente. Si la descripción de alguno de estos campos: **FiscalName** , **CommercialName** , **Address** , **Town** o **Province** excede
los 40 caracteres, la rutina recortará los campos a los primeros 40 caracteres. Si la descripción del campo **PostCode** excede los 10
caracteres, la rutina recortará el campo a los primeros 10 caracteres. Si la descripción de alguno de estos campos: **LandLine** ,
**MobilePhone** o **FIN** excede los 15 caracteres, la rutina recortará los campos a los primeros 15 caracteres. Si la descripción del campo
**EMail** excede los 60 caracteres, la rutina recortará el campo a los primeros 10 caracteres. Si el campo **PerDiscount** tiene un valor
incorrecto (menor que 0,00 o mayor que 99,99), se generará un error y se informará de ello. Si el valor tuviese más de 2 decimales,
será redondeado a 2 decimales. Si alguno de los siguientes códigos: **PaymentCode** , **Representative** , **AreaCode** , **TAVCode** o **RateCode**
no existiese en su correspondiente tabla de la base de datos, se generará un error y se informará de ello. Cuando se está creando
un nuevo cliente, el programa no puede estar trabajando con el mantenimiento de clientes. Si es así, se generará un error de bloqueo
y se informará de ello.

##### Ruta de llamada

/API/Customers/Create

##### Parámetros de entrada

**Code** **_(integer)_** : Valor entero de 1 a 6 dígitos mayor que cero en el que se deberá especificar el código del cliente que se desea crear.
Si se especifica el 0, se considerará autonumérico En ese caso, la aplicación asignará el código de cliente a ser creado. Para ello,
buscará el último código de cliente creado y le sumará 1 (siempre que sea posible).

**FiscalName** **_(string)_** : Cadena de texto de máximo de 40 caracteres en la que se debe especificar el nombre fiscal del cliente a ser
creado.

**CommercialName** **_(string)_** : Cadena de texto de máximo de 40 caracteres en la que se debe especificar el nombre comercial del
cliente a ser creado.

**Address** **_(string)_** : Cadena de texto de máximo de 40 caracteres en la que se puede especificar la dirección del cliente a ser creado.

**PostCode** **_(string)_** : Cadena de texto de máximo de 10 caracteres en la que se puede especificar el código postal de la población del
cliente a ser creado.

**Town** **_(string)_** : Cadena de texto de máximo de 40 caracteres en la que se puede especificar la población del cliente a ser creado.

**Province** **_(string)_** : Cadena de texto de máximo de 40 caracteres en la que se puede especificar la provincia del cliente a ser creado.

**LandLine** **_(string)_** : Cadena de texto de máximo de 15 caracteres en la que se puede especificar el teléfono fijo del cliente a ser creado.

**MobilePhone** **_(string)_** : Cadena de texto de máximo de 15 caracteres en la que se puede especificar el teléfono móvil del cliente a ser
creado.

**FIN** **_(string)_** : Cadena de texto de máximo de 15 caracteres en la que se puede especificar el identificador fiscal del cliente a ser creado.

**FINType** **_(string)_** : Campo de tipo numérico que permitirá especificar el tipo de documento de identificación del cliente. Los tipos de
documentos disponibles son:

```
1 = N.I.F.
2 = N.I.F. Extranjero
3 = Pasaporte
4 = ID en País de Residencia
```

```
5 = Certificado Residencia
6 = Otro Documento
```
**Email** **_(string)_** : Cadena de texto de máximo de 60 caracteres en la que se puede especificar el E-Mail del cliente a ser creado.

**PerDiscount** **_(decimal)_** : Valor decimal entre 0,00 y 99,99 (máximo de 2 decimales), con el que se podrá especificar el porcentaje de
descuento aplicable al cliente.

**PaymentMode** **_(integer)_** : Valor entero entre 1 y 99 con el que se podrá especificar el Código de la Forma de Pago a ser asignada al
cliente. Si la aplicación corresponde a una con módulo de gestión, el valor de este campo no puede ser cero.

**Representative** **_(integer)_** : Valor entero entre 1 y 9999 con el que se podrá especificar el Código de Representante a ser asignado al
cliente. Si la aplicación corresponde a una con módulo de gestión, el valor de este campo no puede ser cero.

**AreaCode** **_(integer)_** : Valor entero entre 1 y 999 con el que se podrá especificar el Código de Zona a ser asignada al cliente. Si la
aplicación corresponde a una con módulo de gestión, el valor de este campo no puede ser cero.

**TAVCode** **_(integer)_** : Valor entero entre 1 y 99 con el que se podrá especificar el Código de a ser asignado al cliente. Si la aplicación
corresponde a una con módulo de gestión, el valor de este campo no puede ser cero.

**RateCode** **_(integer)_** : Valor entero entre 1 y 99 con el que se podrá especificar el Código de Tarifa a ser asignada al cliente. (Si la
aplicación corresponde a una con módulo de gestión, el valor de este campo no puede ser cero.

**Overwrite** **_(boolean)_** : Campo lógico con el que se podrá indicar si se desea sobrescribir un cliente en el caso de que ya exista en la
base de datos (True: Sí sobrescribir; False: No sobrescribir).

##### JSON de solicitud

{
"Code": 33,
"FiscalName": "CLIENTE DE GESTIÓN",
"CommercialName": "CLI. GEST.",
"Address": "C/ MAYOR, 21",
"PostCode": "17520",
"Town": "PUIGCERDA",
"Province": "GIRONA",
"LandLine": "972 882 000",
"MobilePhone": "609 33 66 55",
"FIN": "11223344-Y",
"FINType": 0.0,
"Email": "cliauto@cliauto.com",
"PerDiscount": 15.0,
"PaymentMode": 1,
"Representative": 1,
"AreaCode": 1,
"TAVCode": 1,
"RateCode": 1,
"Overwrite": true
}

##### Parámetros de salida

**ErrorMessage** **_(string)_** : Cadena que almacenará posibles errores. Si la consulta se realizó correctamente, estará vacía.

##### JSON de respuesta

{


"ErrorMessage": ""
}


## Categoría Comandas

En esta categoría se incluirán todos los comandos que permitirán interactuar con datos de comandas. Los comandos disponibles
para esta categoría serán los siguientes:

1. **CreateOrder** : Crea una nueva comanda en la base de datos.
2. **GetOrder** : Obtiene el contenido de una comanda.
3. **CancelOrder** : Elimina una comanda en la base de datos.
4. **AddOrderTip** : Añade la propina a una comanda.
5. **AddOrderPayment** : Añade un pago a una comanda.
6. **InvoiceOrder** : Emite la factura de una comanda.


### CreateOrder

Comando que permitirá crear una nueva comanda (según el valor de **Type** : ticket aparcado, mesa o comanda de servicio a domicilio)
en el terminal indicado. Las mesas solo se podrán crear en las aplicaciones de Hostelería, Servicio a Domicilio, Pubs y Discotecas,
Panaderías y Pastelerías y Panaderías y Pastelerías con Gestión. Las comandas de servicio a domicilio sólo se podrán crear en la
aplicación de Servicio a Domicilio. De la misma manera sólo se podrán añadir suplementos y comentarios a las líneas de la comanda
en las aplicaciones que contemplen esta funcionalidad (las mismas que permiten trabajar con mesas). Si el valor **OrderEndType** es
0 la comanda será creada directamente como tal (según el valor del campo **Type** ), contenga lo que contenga. Si, por el contrario, se
quiere que el usuario de BDP-Net (el camarero o el vendedor) revise la comanda antes de darla por definitiva, se tendrá que indicar
el valor de **OrderEndType** a 1 (las comandas que lleguen por esta vía a la aplicación lo harán a través de la consola de autocomanda).
Es importante que los importes totales de las líneas a ser creadas para la comanda estén bien calculados y que se haga en el mismo
orden que lo hace BDP-Net con sus comandas. El criterio para calcular los totales de la comanda a ser creada son los siguientes:

1. El total de la comanda es igual a la suma de totales de las líneas menos el descuento general.
2. El total de una línea es igual al producto de las unidades de la línea por el precio unitario del artículo, más el
    suplemento de la línea, menos el descuento de la línea.
3. El total de los suplementos de una línea es igual a la suma de los totales de las líneas de suplemento de ésta.
4. El total de la línea de suplementos es igual al producto de las unidades del suplemento por el precio unitario.

##### Ruta de llamada

/API/Orders/Create

##### Parámetros de entrada

**EmployeeId** **_(integer)_** : Código de empleado de la comanda. Será asignado como el empleado de cada una de las líneas de la
comanda. Debe ser un empleado válido (tiene que estar dado de alta) en la base de datos de BDP-Net.

**ItemsProfileId** **_(integer)_** : Código del perfil de departamentos y artículos a usar, como un terminal cualquiera de BDP-Net, para
consultar información de los artículos como, por ejemplo, las impresoras auxiliares.

**OrderEndType** **_(integer)_** : Tipo de finalización de la comanda. Puede tener los siguientes valores:

```
0 : Comanda autoaceptada. Finalización automática. Se da por buena la comanda recibida, sea cual sea su contenido,
aparcando y enviando a cocina el pedido.
1 : Pendiente de validación. Se finaliza como una comanda intermedia que el usuario tendrá que validar en la pantalla de
ventas del TPV.
```
**Order** **_(Order)_** : Objeto que contiene los datos de la comanda. Es una estructura del tipo _Order_.

**OrderOperationType (** **_integer_** **):** Campo de tipo numérico que permitirá especificar el tipo de operación a realizar con este comando.
Dispone de dos valores.

```
0 : CheckAndCreateOrder (valor por defecto). La aplicación chequeará, y si es posible, creará la comanda.
1 : OnlyCheck. La aplicación chequeará si se puede crear la comanda y devolverá el resultado, pero no la creará.
```
**Invoice** **_(boolean)_** : Opcional. Indica si se facturará la comanda después de crearla en el caso que los pagos igualen el total. Si después
de crear la comanda y añadir los pagos indicado en _Payments_ (dentro de _Order_ ) el total pagado no es igual al total de la comanda
(es decir, si queda una parte pendiente), no se facturará la comanda.

**InvoiceParameters** **_(InvoiceParameters)_** : Es opcional y sólo aplica cuando se indica _Invoice_ a _true_. Estructura de tipo
_InvoiceParameters_ con los parámetros para facturar la comanda.


##### JSON de solicitud

{
"EmployeeId": 1,
"ItemsProfileId": 1,
"OrderEndType": 0,
"Order": {
"MarketplaceOrderId": "123456789",
"MarketId": 1,
"MarketName": "Restaurante BDP Test",
"PreparationTime": "2020- 09 - 01T16:24:35Z",
"OrderId": 0,
"PosId": 1,
"Type": 2,
"RoomNumber": 0,
"TableNumber": 0,
"Customer": {
"Id": 0,
"Name": "Cliente Test 1 (WEB)",
"AddressStreet": "c\\ del río",
"AddressNumber": "1",
"ZipCode": null,
"City": null,
"Region": null,
"Country": null,
"LandlineNumber": "555 11111111",
"MobileNumber": "111666111",
"Email": null,
"TaxId": "1234567Q",
"TaxType": 1
},
"Items": [
{
"Lin": 1,
"Id": 7002,
"Name": "HAMBUGUESA CON TOMATE F.F.",
"Units": 1.0,
"Price": 3.01,
"Supplement": 3.61,
"Discount": 0.0,
"DiscountPct": false,
"Total": 6.62,
"VatPct": 10.0,
"Comments": [
{
"Id": 8,
"Name": "CON SALSA ROSA",
"Units": 1.0
},
{
"Id": 4,
"Name": "CON ENSALADA",
"Units": 1.0
}
],
"Supplements": [
{
"Id": 11013,
"Name": "PIMIENTOS",
"Units": 1.0,
"Price": 3.61,
"Total": 3.61,
"Comments": [
{
"Id": 99,
"Name": "Pimientos rojos",


"Units": 1.0
}
]
}
],
"OrderItemType": 0,
"OrderItemTypeMetaInfo": "",
"TyC_D1": 0,
"TyC_D2": 0,
"TyC_D3": 0,
"OnSale": false
},
{
"Lin": 2,
"Id": 19001,
"Name": "PIZZA MARGARITA",
"Units": 1.0,
"Price": 6.01,
"Supplement": 0.0,
"Discount": 0.0,
"DiscountPct": false,
"Total": 6.01,
"VatPct": 10.0,
"Comments": [],
"Supplements": [],
"OrderItemType": 2,
"OrderItemTypeMetaInfo":"{\"FixedItems\":[{\"Id\":314,\"Description\":\"TOMATE
KG\",\"Units\":1.0},{\"Id\":207,\"Description\":\"QUESO RALLADO
KG\",\"Units\":1.0},{\"Id\":354,\"Description\":\"JAMÓN DULCE
KG\",\"Units\":1.0},{\"Id\":358,\"Description\":\"OREGANO BOTE
KG\",\"Units\":1.0}],\"VariableItems\":[{\"Id\":334,\"Description\":\"PIÑA NATURAL
KG\",\"Units\":1.0},{\"Id\":345,\"Description\":\"ACEITUNAS RELLENAS KG\",\"Units\":1.0}]}",
"TyC_D1": 0,
"TyC_D2": 0,
"TyC_D3": 0,
"OnSale": false
},
{
"Lin": 3,
"Id": 18001,
"Name": "MENÚ DEL DÍA",
"Units": 1.0,
"Price": 10.02,
"Supplement": 1.0,
"Discount": 0.0,
"DiscountPct": false,
"Total": 3.0,
"VatPct": 10.0,
"Comments": [],
"Supplements": [],
"OrderItemType": 1,
"OrderItemTypeMetaInfo": "{\"Items\":[{\"Group\":1,\"Id\":100026,\"Description\":\"PAELLA MIXTA
MENU\",\"Units\":1.0,\"Supplement\":1.0},{\"Group\":2,\"Id\":100038,\"Description\":\"CALAMARES EN SU TINTA
MENU\",\"Units\":1.0,\"Supplement\":0.0},{\"Group\":3,\"Id\":100043,\"Description\":\"MACEDONIA FRUTAS
MENU\",\"Units\":1.0,\"Supplement\":0.0},{\"Group\":4,\"Id\":1001,\"Description\":\"Coca-
Cola\",\"Units\":1.0,\"Supplement\":0.0}]}",
"TyC_D1": 0,
"TyC_D2": 0,
"TyC_D3": 0,
"OnSale": false
}
],
"Discount": 0.0,
"DiscountPct": false,
"Total": 23.65,
"ExecutionTime": "2020- 09 - 01T16:54:35Z",


"Status": 0,
"Comments": "",
"Payments": [
{
"TenderId": 1,
"Amount": 23.65,
"PaymentId": "PayPal"
}
]
},
"OrderOperationType": 0
}

##### Parámetros de salida

**OrderId** **_(integer)_** : Número de comanda generada.

**InvoiceNumber** **_(string)_** : Número de la factura emitida.

**ErrorMessage** **_(string)_** : Cadena de errores. Si todo ha ido bien, estará vacía.

Si no se puede crear la comanda, se devolverá el error en el campo **ErrorMessage**.

Si se quería crear una comanda sin facturarla se devolverá el identificador de la comanda interno de BDP en el campo **OrderId**.

Si lo que se quería era crear una comanda y facturarla, se devolverá el número de factura emitida en **InvoiceNumber**. Si se ha creado
la comanda pero, por algún motivo, no se ha podido facturar, se devolverá el campo **OrderId** para poderla facturar posteriormente
usando la función **InvoiceOrder**.

##### JSON de respuesta

{
"OrderId": 30,
"ErrorMessage": ""
}


### GetOrder

Devuelve a través del objeto _Order_ el contenido de la comanda solicitada. En el caso en que se quiera obtener una comanda que ya
ha sido facturada la estructura _Order_ estará vacía pero el campo _Status_ tendrá el valor 3 (comanda facturada). La comanda a devolver
se identificará por el campo _OrderId_ (código que se habrá obtenido previamente en la respuesta de _CreateOrder_ ). En su defecto se
podrá obtener la comanda usando el par de campos _MarketId_ y _MarketplaceOrderId_ (que se habrán pasado previamente a través
de la llamada a _CreateOrder_ ). En su defecto y siempre y cuando se trate de la comanda de una mesa, se podrá obtener usando el
par de campos _RoomNumber_ y _TableNumber_.

##### Ruta de llamada

/API/Orders/Get

##### Parámetros de entrada

**OrderIdentifier** **_(OrderIdentifier)_** : Objeto de tipo _OrderIdentifier_ que contiene los parámetros que identifican a la comanda.

##### JSON de solicitud

{
"OrderIdentifier": {
"OrderId": 1
}
}

{
"OrderIdentifier": {
"MarketId": 25,
"MarketplaceOrderId": "JCY50812"
}
}
{
"OrderIdentifier": {
"RoomNumber": 2,
"TableNumber": 7
}
}

##### Parámetros de salida

**Order** **_(order)_** : Estructura de tipo _Order_ con el contenido de la comanda.

**Status** **_(integer)_** : Estado de la comanda. Mismos valores que el campo _Status_ de la estructura _Order_.

**ErrorMessage** **_(string)_** : Descripción del posible error.

##### JSON de respuesta

{
"Order": { ... (misma estructura que el objeto _Order_ de entrada en el apartado anterior) },
"Status": 1,
"ErrorMessage": ""
}


### CancelOrder

Mediante este comando se ofrece la posibilidad de eliminar una comanda de la aplicación. La comanda a cancelar se identificará a
través del objeto **_OrderIdentifier_** (de la misma manera que en **GetOrder** ). Si correspondiera, se enviarían las anulaciones de la
comanda a impresoras auxiliares utilizando la configuración de impresoras auxiliares del terminal especificado en el parámetro de
entrada _PosId_.

##### Ruta de llamada

/API/Orders/Cancel

##### Parámetros de entrada

**PosId** **_(integer)_** : código del terminal de BDP-Net desde el que se cancelará la comanda. Debe ser un terminal válido (tiene que estar
dado de alta) en la configuración de BDP-Net.

**OrderIdentifier** **_(OrderIdentifier)_** : Objeto de tipo _OrderIdentifier_ que contiene los parámetros que identifican a la comanda.

##### JSON de solicitud

{
"PosId": 1,
"OrderIdentifier": {
"OrderId": 1
}
}

{
"PosId": 1,
"OrderIdentifier": {
"MarketId": 25,
"MarketplaceOrderId": "JCY50812"
}
}

{
"PosId": 1,
"OrderIdentifier": {
"RoomNumber": 2,
"TableNumber": 7
}
}

##### Parámetros de salida

**ErrorMessage** **_(string)_** : Cadena de errores. Si todo ha ido bien, estará vacía.

##### JSON de respuesta

{
"ErrorMessage": ""
}


### AddOrderTip

Esta función permite añadir la propina a una comanda. La comanda se identificará a través del objeto **_OrderIdentifier_** (de la misma
manera que en **GetOrder** ).

Si no se establece el parámetro _AddTip_ a _true_ , el importe de la propina especificado en _Amount_ sustituirá a la propina que pudiera
tener la comanda. Por el contrario si _AddTip_ es _true_ se sumarán las propinas. El importe de la propina resultante puede ser menor
que la antigua propina o incluso cero (para eliminarla) siempre y cuando el nuevo total con propina no sea inferior al total de los
pagos que pueda tener la comanda.

##### Ruta de llamada

/API/Orders/Tip/Add

##### Parámetros de entrada

**OrderIdentifier** **_(OrderIdentifier)_** : Objeto de tipo _OrderIdentifier_ que contiene los parámetros que identifican a la comanda.

**Amount** **_(decimal)_** : Valor de la popina a establecer

**AddTip** **_(boolean)_** : Se puede establecer a _true_ para que el importe especificado en _Amount_ se sume a la propina que ya tenía la
comanda, o bien a _false_ para sustituirla. Si no se especifica este parámetro, su valor por defecto será _false_.

##### JSON de solicitud

{
"OrderIdentifier": {
"RoomNumber": 1,
"TableNumber": 2
},
"Amount": 4.45,
"AddTip": false
}

##### Parámetros de salida

**ErrorMessage** **_(string)_** : Cadena de errores. Si todo ha ido bien, estará vacía.

##### JSON de respuesta

{
"ErrorMessage": ""
}


### AddOrderPaymet

Esta función permite añadir pagar total o parcialmente una comanda. La comanda se identificará a través del objeto **_OrderIdentifier_**
(de la misma manera que en **GetOrder** ).

##### Ruta de llamada

/API/Orders/Payment/Add

##### Parámetros de entrada

**OrderIdentifier** **_(OrderIdentifier)_** : Objeto de tipo _OrderIdentifier_ que contiene los parámetros que identifican a la comanda.

**Payment** **_(OrderPayment)_** : Objeto con los detalles del pago. Es una estructura de tipo _OrderPayment_.

**Invoice** **_(boolean)_** : Opcional. Indica si se facturará la comanda tras hacer el pago y en el caso que los pagos igualen el total. Si después
de añadir el pago indicado en _Payment_ el total pagado de la comanda no es igual al total de la comanda (es decir, si queda una parte
pendiente), no se facturará la comanda.

**PosId** **_(integer)_** : Necesario sólo cuando se indica _Invoice_ a _true_. Código del terminal de BDP-Net desde el que se facturará la comanda.

**EmployeeId** **_(integer)_** : Necesario sólo cuando se indica _Invoice_ a _true_. Código de empleado que será asignado como el empleado
que ha cerrado la comanda.

**InvoiceParameters** **_(InvoiceParameters)_** : Es opcional y sólo aplica cuando se indica _Invoice_ a _true_. Estructura de tipo
_InvoiceParameters_ con los parámetros para facturar la comanda.

##### JSON de solicitud

{
"OrderIdentifier": {
"RoomNumber": 1,
"TableNumber": 2
},
"Payment": {
"TenderId": 2,
"Amount": 75.00,
"PaymentId": "HsWJ2785s"
}
}

##### Parámetros de salida

**InvoiceNumber** **_(string)_** : Número de la factura emitida.

**ErrorMessage** **_(string)_** : Cadena de errores. Si todo ha ido bien, estará vacía.

##### JSON de respuesta

{
"InvoiceNumber": "00001TB028469"
"ErrorMessage": ""
}


### InvoiceOrder

Función para facturar una comanda que se identificará a través del objeto **_OrderIdentifier_** (de la misma manera que en **GetOrder** ).

##### Ruta de llamada

/API/Orders/Invoice

##### Parámetros de entrada

**PosId** **_(integer)_** : Código del terminal de BDP-Net desde el que se facturará la comanda.

**EmployeeId** **_(integer)_** : Código de empleado que será asignado como el empleado que ha cerrado la comanda.

**OrderIdentifier** **_(OrderIdentifier)_** : Objeto de tipo _OrderIdentifier_ que contiene los parámetros que identifican a la comanda.

**InvoiceParameters** **_(InvoiceParameters)_** : Opcional. Estructura de tipo _InvoiceParameters_ con los parámetros para facturar la
comanda.

##### JSON de solicitud

{
"PosId": 1,
"EmployeeId": 1,
"OrderIdentifier": {
"RoomNumber": 1,
"TableNumber": 2
},
"InvoiceParameters": {
"InvoiceEmailAddress": "test@te.st",
"PrintTicket": false,
"BillingDetails": {
"Name": "Empresa SL",
"Address": "C. Alfons I 31, baixos",
"ZipCode": "17520",
"City": "Puigcerdà",
"TaxId": "B-12345678",
"TaxType": 1,
}
}
}

##### Parámetros de salida

**InvoiceNumber** **_(string)_** : Número de la factura emitida.

**ErrorMessage** **_(string)_** : Cadena de errores. Si todo ha ido bien, estará vacía.

##### JSON de respuesta

{
"InvoiceNumber": "00001TB028469"
"ErrorMessage": ""
}


## Categoría Comentarios

En esta categoría se incluirán todos los comandos que permitirán interactuar con datos de comentarios. Los comandos disponibles
para esta categoría será el siguiente:

1. **GetCommetsProfile** : Obtiene la información de un perfil de comentarios, así como de sus grupos de comentarios.


### GetCommentsProfile

Mediante este comando se obtendrá una estructura de árbol con todos los perfiles de comentarios con código comprendidos entre
**_Profile1_** y **_Profile2_** , cada uno con sus grupos de comentarios y los comentarios de todos estos.

##### Ruta de llamada

/API/CommentsProfiles/Get

##### Parámetros de entrada

**Profile1** **_(integer)_** : Número entero entre 1 y 999. Código inicial de perfil de comentarios a devolver. Si no se especifica, se devolverán
todos los perfiles desde el primero hasta el especificado en **Profile2**.

**Profile2** **_(integer)_** : Número entero entre 1 y 999. Código final de perfil de comentarios a devolver. Si no se especifica, se devolverán
todos los perfiles desde el especificado en **Profile1** hasta el último.

##### JSON de solicitud

{
"Profile1": 1,
"Profile2": 0
}

##### Parámetros de salida

**ErrorMessage** **_(string)_** : Cadena de texto con la descripción del posible error en la consulta.

**CommentsProfiles** **_List (of CommentsProfile)_** : Lista de perfiles de comentarios obtenidos. Es un conjunto de estructuras del tipo
**CommentsProfile** , la cual está formada por los siguientes elementos:

```
Id (integer) : Código del perfil de comentarios.
```
```
Name (string) : Descripción del perfil de comentarios.
```
```
Groups List(of CommentsGroup) : Lista de conjunto de grupos de comentarios del perfil. Es una lista de estructuras del tipo
CommentsGroup , la cual está formada por los siguientes elementos:
```
```
Id (integer) : Código del grupo de comentarios.
```
```
Name (string) : Descripción del grupo de comentarios.
```
```
Comments List(of Comments) : Lista de comentarios del grupo. Es un conjunto de estructuras del tipo Comment ,
la cual está formada por los siguientes elementos:
```
```
Id (integer) : Código del comentario.
```
```
Name (string) : Descripción del comentario.
```
##### JSON de respuesta

{
"ErrorMessage": "",


"CommentsProfiles": [
{
"Id": 1,
"Name": "COMENTARIOS CARNES",
"Groups": [
{
"Id": 1,
"Name": "TIPO DE PREPARACIÓN",
"Comments": [
{
"Id": 1,
"Name": "POCO HECHO"
},...
{
"Id": 7,
"Name": "CON SAL"
}
]
},...
{
"Id": 5,
"Name": "ACOMPAÑAMIENTOS",
"Comments": [
{
"Id": 4,
"Name": "CON ENSALADA"
},
{
"Id": 5,
"Name": "CON PATATAS"
}
]
}
]
},...
{
"Id": 3,
"Name": "COMENTARIOS FASTFOOD",
"Groups": []
}
]
}


## Categoría Departamentos

En esta categoría se incluirán todos los comandos que permitirán interactuar con datos de departamentos. Los comandos
disponibles para esta categoría serán los siguientes:

1. **ExportDepartment** : Obtiene una lista de departamentos.
2. **DepartmentsExportFromProfile** : Obtiene una lista de departamentos con la información del perfil de artículos solapada.
3. **CreateDepartment** : Crea un departamento en la base de datos.
4. **CreateDepartmentAndupdateProfiles** : Crea un departamento en la base de datos de departamentos y lo dará de alta en
    uno o varios perfiles de artículos.


### ExportDepartments

Este comando devuelve una lista de departamentos según los criterios de filtrado y de ordenación especificados en los parámetros
de entrada. También pueden paginarse los resultados. Si no se desea paginación, el valor de ItemsPerPage se deberá especificar un

0. Se puede especificartambién por qué campo de ordenan los resultados y si se hace en orden ascendente o descendente. Si los
campos Dept1 y Dept2 tienen valor correcto asignado, se buscará por código. Si el campo Description contiene algo, se buscará por
descripción. Si se busca por descripción, podrá buscarse indicando que contenga la cadena Descripcion, que empiece por ella o que
acabe por ella. Los resultados podrán paginarse. Hay que indicar cuántos elementos componen la página (ItemsPerPage) y qué
página deseamos que se nos muestre (ActualPage). Si (ItemsPerPage) vale 0, es que no hay paginación.

##### Ruta de llamada

/API/Departments/Export

##### Parámetros de entrada

**Dept1** **_(integer)_** : Código del departamento inicial. Número entero entre 1 y 999.

**Dept2** **_(integer)_** : Código del departamento final. Número entero entre 1 y 999.

**Description** **_(string)_** : Cadena de texto con el nombre del departamento.

**DescriptionQueryType** **_(StringSearchType)_** : Valor numérico con el que se podrá indicar el tipo de búsqueda que se realizará si se
busca por descripción. Los tipos disponibles son los siguientes:

```
None = 0 : ningún tipo.
Starts = 1 : que empiece por.
Contains = 2 : que contenga.
Ends = 3 : que acabe por.
```
**ItemsPerPage(** **_integer_** **)** : Valor numérico que permitirá, en caso de querer que los resultados se den paginados, cuántos elementos
tiene cada página.

**ActualPage(** **_integer_** **)** : Valor numérico que permitirá, en caso de querer que los resultados se den paginados, la actual página.

**nField(** **_GetDepartmentField_** **)** : Valor numérico con el que se podrá indicar por qué campo se ordenarán los resultados. Los tipos
disponibles son:

```
DeptCode = 1 : Por código de departamento.
DeptDescription = 2 : Por descripción del departamento.
```
**nOrder(** **_OrderType_** **)** : Valor numérico que permitirá especificar cómo se ordenarán los resultados: si de manera ascendente o
descendente. Los tipos disponibles son:

```
ASC = 0 : Ascendente (de menor a mayor).
DESC = 1 : Descendente (de mayor a menor).
```
##### JSON de solicitud

{
"Dept1": 0,
"Dept2": 999,
"Description": "",
"DescriptionQueryType": 2,


"ItemsPerPage": 0,
"ActualPage": - 1,
"nField": 1,
"nOrder": 0
}

##### Parámetros de salida

**NumberOfItems(** **_integer_** **)** : Valor numérico que informa del total de elementos de la búsqueda. Si hay paginación, no da el total de
elementos de la página, sino el total de la búsqueda. Por ejemplo: si hay 100 departamentos paginados de 10 en 10, devuelve 100,
no 10.

**ErrorMessage** **_(string)_** : Cadena de errores. Si todo ha ido bien, estará vacía.

**DepartmentListData(** **_List Of DepartmentListDataType_** **)** : Devuelve una lista de departamentos en formato objetos del tipo
DepartmentListDataType. Dicho objeto se compone de los siguientes campos:

```
DeptCode( long ) : Valor numérico entero que indica el código del departamento.
```
```
DeptDescription( string ) : Cadena de texto que muestra el nombre del departamento.
```
##### JSON de respuesta

{
"NumberItems": 31,
"ErrorMessage": "",
"DepartmentListData": [
{
"DeptCode": 1,
"DeptDescription": "REFRESCOS"
},...
{
"DeptCode": 205,
"DeptDescription": "PLATOS COMBINADOS CARTA"
}
]
}


### DepartmentsExportFromProfile

Este comando devuelve una lista de los departamentos que estén dados de alta en un perfil de departamentos y artículos
especificable.

##### Ruta de llamada

/API/Departments/ExportFromProfile

##### Parámetros de entrada

**ProfileId** **_(integer)_** : Valor numérico entero entre 1 y 999 en el que se tendrá que especificar el código de perfil de departamentos y
artículos para el que se quiere obtener sus departamentos.

##### JSON de solicitud

{
"ProfileId": 1
}

##### Parámetros de salida

**DepartamentosListData(** **_List Of DepartamentosDataType_** **)** : Devuelve una lista de departamentos dados de alta en el perfil de
departamentos y artículos especificado en formato objetos del tipo **_Departamento_** **ListDataType**. Dicho objeto se compone de los
siguientes campos:

```
Codigo( long ) : Valor numérico entero que indica el código del departamento.
```
```
Descripcion( string ) : Cadena de texto que muestra el nombre del departamento.
```
```
SubDepartamentos( List Of DepartamentosDataType ) : Devuelve una lista con departamentos asociados al departamento
en cuestión en formato objetos del tipo Departamento ListDataType (objeto ya explicado).
```
##### JSON de respuesta

{
"Departamentos": [
{
"Codigo": 1.0,
"Descripcion": "REFRESCOS",
"SubDepartamentos": []
},...
{
"Codigo": 10.0,
"Descripcion": "VINOS",
"SubDepartamentos": [
{
"Codigo": 103.0,
"Descripcion": "RIOJAS",
"SubDepartamentos": []
},...
{
"Codigo": 106.0,
"Descripcion": "OTRAS DENOMINACIONES",
"SubDepartamentos": []


}
]
},
],
"ErrorMessage": ""
}


### CreateDepartment

Crea un departamento con los parámetros de entrada facilitados. Si ya existiese el departamento, si **Overwrite** =True, sobrescribe el
departamento; en caso contrario, no lo hace y emite un error informando de la existencia del departamento. Si la descripción del
departamento excede los 40 caracteres, la rutina recortará la descripción a los primeros 40 caracteres. Si la descripción abreviada
del departamento excede los 10 caracteres, la rutina recortará la descripción abreviada a los primeros 10 caracteres. Si la descripción
gráfica 1, 2 o 3 del departamento exceden los 7 caracteres, la rutina recortará las descripciones gráficas a los primeros 7 caracteres.
Los demás campos de la ficha de departamentos se inicializarán según los valores por defecto de la base de datos. Cuando se está
creando un nuevo departamento, el programa no puede estar trabajando con el mantenimiento de departamentos. Si es así, se
generará un error de bloqueo y se informará de ello.

##### Ruta de llamada

/API/Departments/Create

##### Parámetros de entrada

**Code** **_(integer)_** : Número entero de 1 a 3 dígitos mayor que cero. Corresponde al código del departamento que se desea crear.

**Description** **_(string)_** : Cadena de texto no vacía con un máximo de 40 caracteres. Corresponde a la descripción o nombre del
departamento que se desea crear.

**ShortDescription** **_(string)_** : Cadena de texto no vacía con un máximo de 10 caracteres. Corresponde a la descripción abreviada del
departamento que se desea crear.

**GraphDescription1** **_(string)_** : Cadena de texto no vacía con un máximo de 7 caracteres. Corresponde a la descripción gráfica 1 del
departamento que se desea crear.

**GraphDescription2** **_(string)_** : Cadena de texto no vacía con un máximo de 7 caracteres. Corresponde a la descripción gráfica 2 del
departamento que se desea crear.

**GraphDescription3** **_(string)_** : Cadena de texto no vacía con un máximo de 7 caracteres. Corresponde a la descripción gráfica 3 del
departamento que se desea crear.

**Overwrite** **_(boolean)_** : Campo booleano (True: Sí; False: No) que indica si deseamos sobrescribir un departamento en el caso de que
ya exista en la base de datos.

##### JSON de solicitud

{
"Code": 77,
"Description": "DEPTO. WEBLINK",
"ShortDescription": "D. WEB.",
"GraphDescription1": "D. WEB.",
"GraphDescription2": "",
"GraphDescription3": "",
"Overwrite": true
}

##### Parámetros de salida

**ErrorMessage** **_(string)_** : cadena de errores. Si todo ha ido bien, estará vacía.


##### JSON de respuesta

{
"ErrorMessage": ""
}


### CreateDepartmentAndUpdateProfiles

Crea un departamento según los valores especificados en los parámetros de entrada y lo añade a los perfiles del terminal
especificados en **ProfileList** , a menos que **AllProfiles** valga **True** , en cuyo caso los añade en todos los posibles.

##### Ruta de llamada

/API/Departments/CreateAndUpdateProfiles

##### Parámetros de entrada

**ProfileList(** **_ProfilesListInfo_** **)** : Lista de perfiles. Estará compuesta por objetos **_ProfileListInfo_**. La estructura de ese tipo de objeto
corresponde a la siguiente:

```
Profile( integer ) : Entero con el código del perfil.
```
```
ProfileName( string ) : Cadena con la descripción del perfil.
```
**AllProfiles(** **_boolean_** **)** : Indica si se añade el departamento en todos los perfiles posibles, ignorando la lista de terminales especificada.

**AutomaticCode(** **_boolean_** **)** : Si se especifica a **True** , ignorará **Code** e irá a buscar el primer código válido disponible de departamento.
Si ya se hubiese llegado a 999, dará un mensaje de error.

**Code(** **_integer_** **)** : Valor entero entre 1 y 999 correspondiente al código de departamento. No puede existir ya en la base de datos.

**Description(** **_string_** **)** : Cadena con un máximo de 40 caracteres, con el nombre del departamento.

**ShortDescription(** **_string_** **)** : Cadena con un máximo de 10 caracteres, con el nombre abreviado del departamento.

**GraphDescription1(** **_string_** **)** : Cadena con un máximo de 7 caracteres, con el nombre de la descripción del gráfico de la línea 1.

**GraphDescription2(** **_string_** **)** : Cadena con un máximo de 7 caracteres, con el nombre de la descripción del gráfico de la línea 2.

**GraphDescription3(** **_string_** **)** : Cadena con un máximo de 7 caracteres, con el nombre de la descripción del gráfico de la línea 3.

**PrinterLevel(** **_integer_** **)** : Valor entero entre 1 y 9 que indica el nivel de impresión del departamento.

##### JSON de solicitud

{
"AllProfiles": false,
"ProfileList": [
{
.Profile = 1,
.ProfileName = "PERFIL 1",
}
],
"AutomaticCode": true,
"ShortDescription": "D. WEB.",
"Description": "Departamento creado desde Weblink",
"GraphDescription1": "DEPT",
"GraphDescription2": "TEST",
"GraphDescription3": "WEBLINK",
"PrinterLevel": 1
}


##### Parámetros de salida

**ListaErroresArticulo(** **_List Of string_** **)** : Lista de errores de validación.

**ErrorMessage(** **_string_** **)** : Si todo ha ido bien, estará vacía. Contiene errores generales de validación.

##### JSON de respuesta

{
"ListaErroresArticulo": [],
"ErrorMessage": ""
}


## Categoría Menús

En esta categoría se incluirán todos los comandos que permitirán interactuar con los datos de menús. Los comandos disponibles
para esta categoría serán los siguientes:

1. **GetMenuDefinition** : Obtiene la definición de un menú.


### etMenuDefinition

Este comando devuelve en **_MenuData_** toda la definición del menú solicitado en el parámetro **_MenuId_** tal y como está configurado
en el TPV.

##### Ruta de llamada

/API/Menus/Get

##### Parámetros de entrada

**MenuId** **_(integer)_** : Número que identifica el menú a devolver.

##### JSON de solicitud

{
"MenuId": 1
}

##### Parámetros de salida

**MenuData** **_(MenuDataType)_** : Definición del menú. Es una estructura del tipo **MenuDataType** , que tiene los siguientes elementos:

```
Id (integer) : Código del menú solicitado.
```
```
Description (string) : Nombre del menú.
```
```
TastingMenu (boolean) : Indica si se trata de un menú degustación. Corresponde al campo Menú Degustación de la
definición del menú de BDP.
```
```
MaxItemsPerDiner (integer) : Número máximo de platos a elegir por comensal. Corresponde al campo Nº Máx.
Platos/Persona de la definición del menú de BDP.
```
```
KitchenSendingMode (integer) : Tipo de envío a cocina. Corresponde al campo Envíos a Cocina de la definición del menú
de BDP (1 por las impresoras del artículo general, 2 por las impresoras de los componentes).
```
```
AllItemsPerDinerMandatory (boolean) : Indica si es obligatorio seleccionar elementos en todos los grupos del menú.
Corresponde al campo Obligatoriedad de entrar todos los platos de la definición del menú de BDP.
```
```
Groups List (of MenuGroupDataType) : Lista con los grupos configurados en el menú (entrantes, segundos platos, postres,
bebidas, ...). Es una estructura del tipo MenuGroupDataType , que tiene los siguientes elementos:
```
```
Id (integer) : Código del grupo de menú.
```
```
Description (string) : Nombre del grupo.
```
```
KitchenPrintOrder (integer) : Orden de impresión en cocina/impresoras auxiliares. Corresponde al campo Nivel
Plato de la definición del menú de BDP.
```
```
Items List (of MenuItemDataType) : Elementos entre los que el usuario puede elegir dentro del grupo (platos a
elegir entre los entrantes, bebidas a elegir en el menú, etc.). Es una estructura del tipo MenuItemDataType , que
tiene los siguientes elementos:
```
```
ArtCode (long) : Código del artículo (plato, bebida, etc.)
```

```
Line (integer) : Número de línea (orden) del plato dentro del grupo.
```
```
Description (string) : Nombre del artículo.
```
```
Supplement1 ... Supplement5 (decimal) : Importe extra, suplemento, que se añadirá al precio del menú
por elegir este artículo, cuando se aplica la tarifa de precios 1...5.
```
**ErrorMessage** **_(string)_** : Descripción del posible error.

##### JSON de respuesta

{
"MenuData": {
"Id": 1,
"Description": "MENÚ DEL DIA",
"TastingMenu": false,
"MaxItemsPerDiner": 4,
"KitchenSendingMode": 1,
"AllItemsPerDinerMandatory": false,
"Groups": [
{
"Id": 1,
"Description": "ENTRANTES",
"KitchenPrintOrder": 1,
"Items": [
{
"Line": 1,
"Description": "ENSALADA VERDE MENU",
"Supplement1": 0.0,
"Supplement2": 0.0,
"Supplement3": 0.0,
"Supplement4": 0.0,
"Supplement5": 0.0,
"ArtCode": 100001
},...
{

```
]
},...
{
```
```
"Line": 6,
"Description": "PAELLA MIXTA MENU",
"Supplement1": 1.0,
"Supplement2": 1.0,
"Supplement3": 1.0,
"Supplement4": 1.0,
"Supplement5": 1.0,
"ArtCode": 100026
}
```
```
"Id": 4,
"Description": "BEBIDAS",
"KitchenPrintOrder": 9,
"Items": [
{
"Line": 20,
"Description": "COCA-COLA",
"Supplement1": 0.0,
"Supplement2": 0.0,
"Supplement3": 0.0,
"Supplement4": 0.0,
"Supplement5": 0.0,
"ArtCode": 1001
},
{
```

"Line": 21,
"Description": "GASEOSA",
"Supplement1": 0.0,
"Supplement2": 0.0,
"Supplement3": 0.0,
"Supplement4": 0.0,
"Supplement5": 0.0,
"ArtCode": 1023
}
]
}
]
},
"ErrorMessage": ""
}


## Categoría Fast-Foods

En esta categoría se incluirán todos los comandos que permitirán interactuar con datos de Fast-Foods. Los comandos disponibles
para esta categoría serán los siguientes:

1. **GetFastfoodDefinition** : Obtiene la definición de un fast-food.


### GetFastfoodDefinition

Este comando devuelve en **_FastfoodData_** toda la definición del fastfood solicitado en el parámetro **_FastfoodId_** tal y como está
configurado en el TPV.

##### Ruta de llamada

/API/FastFoods/Get

##### Parámetros de entrada

**FastfoodId** **_(integer)_** : Valor numérico que identifica el fastfood a devolver.

##### JSON de solicitud

{
"FastfoodId": 1
}

##### Parámetros de salida

**FastfoodData** **_(FastfoodDataType)_** : Definición del fastfood. Es una estructura del tipo **FastfoodDataType** , que tiene los siguientes
elementos:

```
Id (integer) : Código del fastfood solicitado.
```
```
Description (string) : Nombre del fastfood.
```
```
UnitsLimit (integer) : Límite de unidades de elección de ingredientes. Corresponde al campo Unidades de Elección de la
definición del fastfood de BDP.
```
```
UnitsLimited (boolean) : Indica si se limitan la elección de ingredientes a las unidades especificadas. Corresponde al campo
Limitar Unidades de la definición del fastfood de BDP.
```
```
BasePrice1 ... BasePrice5 (decimal) : Precio base del fastfood aplicando el tipo de precio 1...5. Corresponde a los campos
Precio Base 1...5 de la definición del fastfood de BDP.
```
```
PriceRounding (decimal) : Valor múltiplo al que se redondeará, al alza, el precio final del fastfood. Corresponde al campo
Redondear Precio de la definición del fastfood de BDP.
```
```
ApplyArticleDiscount (boolean) : Indica si al precio final se le aplica el descuento configurado en el artículo fastfood.
Corresponde al campo Aplicar Descuentos del Artículo Principal de la definición del fastfood de BDP.
```
```
Items List (of FastfoodItemDataType) : Lista de elementos/ingredientes configurados en el fastfood entre los que el usuario
puede elegir. Es una estructura del tipo FastfoodItemDataType , que tiene los siguientes elementos:
```
```
ArtCode (long) : Código del artículo/ingrediente.
```
```
Line (integer) : Número de línea (orden) del elemento dentro del fastfood.
```
```
Description (string) : Descripción del artículo/ingrediente.
```
```
FixedItem (boolean) : Determina si se trata de un ingrediente fijo del fastfood o, si por el contrario, es un
ingrediente opcional (elegible).
```

```
UnitValue (integer) : Valor unitario del elemento (qué cantidad de unidades suma en el cómputo de las unidades
de elección – UnitsLimit )
```
```
BasePrice (decimal) : Precio a sumar si se elige el elemento dentro del límite de uidades de elección del fastfood.
```
```
ExtraPrice (decimal) : Precio a sumar si se elige el elemento fuera del límite de uidades de elección del fastfood.
```
**ErrorMessage** **_(string)_** : descripción del posible error.

##### JSON de respuesta

{
"FastfoodData": {
"Id": 1,
"Description": "PIZZA MARGARITA",
"UnitsLimit": 3,
"UnitsLimited": false,
"BasePrice1": 5.71,
"BasePrice2": 5.71,
"BasePrice3": 5.71,
"BasePrice4": 5.71,
"BasePrice5": 5.71,
"PriceRounding": 0.0,
"ApplyArticleDiscount": false,
"Items": [
{
"Line": 1,
"Description": "TOMATE KG",
"FixedItem": true,
"UnitValue": 1,
"BasePrice": 0.0,
"ExtraPrice": 0.0,
"ArtCode": 314
},...
{
"Line": 10,
"Description": "JAMON SERRANO KG",
"FixedItem": false,
"UnitValue": 1,
"BasePrice": 0.15,
"ExtraPrice": 0.45,
"ArtCode": 353
}
]
},
"ErrorMessage": ""
}


## Categoría Packs

En esta categoría se incluirán todos los comandos que permitirán interactuar con los datos de packs. Los comandos disponibles para
esta categoría serán los siguientes:

1. **GetPackDefinition** : Obtiene la definición de un pack.


### GetPackDefinition

Este comando devuelve en **_PackData_** toda la definición del pack solicitado en el parámetro **_PackId_** tal y como está configurado en
el TPV.

##### Ruta de llamada

/API/Packs/Get

##### Parámetros de entrada

**Packd** **_(integer)_** : Número que identifica el pack a devolver.

##### JSON de solicitud

{
"PackId": 1
}

##### Parámetros de salida

**PackData** **_(PackDataType)_** : definición del pack. Es una estructura del tipo **PackDataType** , que tiene los siguientes elementos:

```
Id (integer) : Código del pack solicitado.
```
```
Description (string) : Nombre del pack.
```
```
AddItemsPrice (boolean) : Indica si en el cálculo del precio final del pack se suman los precios de los elementos elegidos o
si bien el precio es fijo (el del artículo pack). Corresponde al campo Precio Fijo/Sumar Precio 1 - 5 de la definición del pack de
BDP.
```
```
PriceType (integer) : Tipo de precio 1...5 a aplicar para sumar los precios de los elementos (según el parámetro
AddItemsPrice ).
```
```
Groups List (of PackGroupDataType) : Lista con los grupos configurados en el pack (ensalada, bocadillo, bebida, ...). Es una
estructura del tipo PackGroupDataType , que tiene los siguientes elementos:
```
```
Id (integer) : Código del grupo del pack.
```
```
Description (string) : Nombre del grupo.
```
```
MinItems (integer) : Número de elementos que se deben elegir en este grupo (0 si no hay mínimo, 1 para elegir al
menos algún elemento). Corresponde al campo Obligatorio mínimo un elemento de la definición del pack de BDP.
```
```
MaxItems (integer) : Número total de elementos que se pueden elegir en este grupo. Corresponde al campo
Elementos a Seleccionar de la definición del pack de BDP.
```
```
Items List (of PackItemDataType) : Elementos entre los que el usuario puede elegir dentro del grupo (bocadillos,
bebidas, etc. a elegir en el pack). Es una estructura del tipo PackItemDataType , que tiene los siguientes elementos:
```
```
ArtCode (long) : Código del artículo. Si viene especificado este código de artículo, el siguiente código de
departamento, DepartmentCode , será 0.
```

```
DepartmentCode (integer) : Código de departamento. Si se especifica un departamento, el campo
ArtCode será 0 y en el pack se podrá elegir cualquier artículo de éste.
```
```
Line (integer) : número de línea (orden) del elemento dentro del grupo.
```
```
Description (string) : Nombre del artículo/departamento.
```
```
MinItems (integer) : Unidades mínimas a elegir obligatoriamente de este artículo/departamento. Si
MinItems es igual a MaxItems se asignarán automáticamente esas unidades del elemento.
```
```
MaxItems (integer) : Unidades máximas elegibles de este artículo/departamento. Si MinItems es igual a
MaxItems se asignarán automáticamente esas unidades del elemento.
```
```
AddPrice (boolean) : Indica si, al elegir este artículo/elemento del departamento, su precio es sumado al
precio total del pack, si el campo AddItemsPrice del pack así lo indica.
```
**ErrorMessage** **_(string)_** : Descripción del posible error.

##### JSON de respuesta

{
"PackData": {
"Id": 1,
"Description": "Pack Combo",
"AddItemsPrice": false,
"PriceType": 1,
"Groups": [
{
"Id": 1.0,
"Description": "BEBIDA",
"MinItems": 0,
"MaxItems": 1,
"Items": [
{
"Line": 1,
"DepartmentCode": 0,
"Description": "COCA-COLA",
"MinItems": 0,
"MaxItems": 1,
"AddPrice": false,
"ArtCode": 1001
},...
{

```
]
},...
{
```
```
"Line": 3,
"DepartmentCode": 0,
"Description": "FANTA NARANJA",
"MinItems": 0,
"MaxItems": 1,
"AddPrice": false,
"ArtCode": 1018
}
```
```
"Id": 3.0,
"Description": "Comida",
"MinItems": 0,
"MaxItems": 1,
"Items": [
{
"Line": 7,
"DepartmentCode": 0,
```

"Description": "ENTRECOTTE PLANCHA CARTA",
"MinItems": 0,
"MaxItems": 1,
"AddPrice": false,
"ArtCode": 12016
},...
{
"Line": 9,
"DepartmentCode": 0,
"Description": "POLLO A L'AST CARTA",
"MinItems": 0,
"MaxItems": 1,
"AddPrice": false,
"ArtCode": 12005
}
]
}
]
},
"ErrorMessage": ""
}


## Categoría Fidelización y Puntos

En esta categoría se incluirán todos los comandos que permitirán interactuar con datos del sistema de fidelización y puntos. Los
comandos disponibles para esta categoría serán los siguientes:

1. **GetPoints** : Obtiene los puntos del sistema de fidelización y puntos de un cliente.
2. **AddPoints** : Permite añadir puntos del sistema de fidelización y puntos a un cliente.


### GetPoints

Este comando devuelve los puntos totales del sistema de fidelización y puntos de un cliente.

##### Ruta de llamada

/API/Loyalty/GetPoints

##### Parámetros de entrada

**Customer** **_(integer)_** : Número entero de 1 a 6 dígitos mayor que cero. Corresponde al código del cliente del que se van a actualizar
los puntos.

##### JSON de solicitud

{
"Customer": 1
}

##### Parámetros de salida

**Points** **_(decimal)_** : Puntos totales del cliente especificado.

**ErrorMessage** **_(string)_** : Cadena de errores. Si todo ha ido bien, estará vacía.

##### JSON de respuesta

{
"Points": 0.0,
"ErrorMessage": ""
}


### AddPoints

Este comando permitirá sumar/restar puntos del sistema de fidelización y puntos al cliente especificado, con la fecha y el motivo
indicados. Si los puntos son positivos, se suman; si son negativos, se restan. Como fecha de última actualización, pone
automáticamente la fecha del sistema.

##### Ruta de llamada

/API/Loyalty/AddPoints

##### Parámetros de entrada

**Customer** **_(integer)_** : Número entero de 1 a 6 dígitos mayor que cero. Corresponde al código del cliente del que se van a actualizar
los puntos.

**PointsAdded** **_(string)_** : Cantidad de puntos a añadir al saldo del cliente.

**Reason** **_(string)_** : Cadena para especificar el motivo por el cual se añaden puntos.

##### JSON de solicitud

{
"Customer": 1,
"PointsAdded": 7.0,
"Reason": "GRATIFICACIÓN"
}

##### Parámetros de salida

**ErrorMessage** **_(string)_** : cadena de errores. Si todo ha ido bien, estará vacía.

##### JSON de respuesta

{
"ErrorMessage": ""
}


## Categoría Terminales

En esta categoría se incluirán todos los comandos que permitirán interactuar con datos de los terminales. Los comandos disponibles
para esta categoría serán los siguientes:

1. **GetPOS** : Obtiene un terminal.
2. **GetPOSes** : Obtiene varios/todos los terminales.


### GetPOS

Este comando devuelve una estructura con datos de un terminal.

##### Ruta de llamada

/API/POS/Get

##### Parámetros de entrada

**Id** **_(integer)_** : Código del terminal que se quiere consultar.

##### JSON de solicitud

{
"Id": 1
}

##### Parámetros de salida

**ErrorMessage** **_(string)_** : Cadena de errores. Si todo ha ido bien, estará vacía.

**POS** **_(POS)_** : Estructura de tipo **POS** con parámetros del terminal.

##### JSON de respuesta

{
"ErrorMessage": "",
"POS": {
"Id": 1,
"Name": "Terminal 1"
}
}


### GetPOSes

Este comando devuelve una lista de estructuras con datos de todos los terminales.

##### Ruta de llamada

/API/POSes/Get

##### Parámetros de entrada

Este comando no tiene parámetros de entrada.

##### JSON de solicitud

{
}

##### Parámetros de salida

**ErrorMessage** **_(string)_** : Cadena de errores. Si todo ha ido bien, estará vacía.

**POSes** **_List (of POS)_** : Lista de terminales obtenidos. Es un conjunto de estructuras de tipo **POS**.

##### JSON de respuesta

{
"ErrorMessage": "",
"POSes": [
{
"Id": 1,
"Name": "Barra"
},
{
"Id": 2,
"Name": "Terraza"
}
]
}


## Categoría Empleados

En esta categoría se incluirán todos los comandos que permitirán interactuar con datos de los empleados. Los comandos disponibles
para esta categoría serán los siguientes:

1. **GetEmployee** : Obtiene un empleado.
2. **GetEmployees** : Obtiene varios/todos los empleados.
3. **GetPOSEmployees** : Obtiene los empleados que se pueden seleccionar en un terminal.


### GetEmployee

Este comando devuelve una estructura con datos de un empleado.

##### Ruta de llamada

/API/Employee/Get

##### Parámetros de entrada

**Id** **_(integer)_** : Código del empleado que se quiere consultar.

##### JSON de solicitud

{
"Id": 1
}

##### Parámetros de salida

**ErrorMessage** **_(string)_** : Cadena de errores. Si todo ha ido bien, estará vacía.

**Employee** **_(Employee)_** : Estructura de tipo **Employee** con datos del empleado.

##### JSON de respuesta

{
"ErrorMessage": "",
" Employee": {
"Id": 1,
"Name": "Camarero uno",
"Salesperson": true
}
}


### GetEmployees

Este comando devuelve una lista de estructuras con datos de los empleados solicitados.

##### Ruta de llamada

/API/Employees/Get

##### Parámetros de entrada

**Ids** **_List (of integer_** **)** : Lista con los códigos de empleados a obtener. Si no se indica ninguno, se devolverán TODOS los empleados
configurados en BDP-Net.

**OnlySalespeople** **_(boolean)_** : Indica si se quieren obtener sólo los empleados configurados en BDP como vendedores/camareros.

##### JSON de solicitud

{
"Ids": [1, 3]
}

{
"OnlySalespeople": true
}

{
}

##### Parámetros de salida

**ErrorMessage** **_(string)_** : Cadena de errores. Si todo ha ido bien, estará vacía.

**Employees** **_List (of Employee)_** : Lista de empleados obtenidos. Es un conjunto de estructuras de tipo **Employee**.

##### JSON de respuesta

{
"ErrorMessage": "",
"Employees": [
{
"Id": 1,
"Name": "Camarero 01",
"Salesperson": true
},
{
"Id": 3,
"Name": "Camarero 03",
"Salesperson": true
}
]
}


### GetPOSEmployees

Este comando devuelve una lista de estructuras con datos de los empleados asociados a un terminal. Si el terminal está configurado
para poder asignar cualquier empleado, se devolverá una lista con todos los empleados vendedores.

##### Ruta de llamada

/API/POS/Employees/Get

##### Parámetros de entrada

**POSId** **_(integer)_** : Código del terminal del que se van a obtener los empleados.

##### JSON de solicitud

{
"POSId": 1
}

##### Parámetros de salida

La respuesta de esta función tiene la misma estructura que la de la función **GetEmployees**.

##### JSON de respuesta

{
"ErrorMessage": "",
"Employees": [
{
"Id": 101,
"Name": "Camarero 101",
"Salesperson": true
},
{
"Id": 102,
"Name": "Camarero 102",
"Salesperson": true
},
{
"Id": 105,
"Name": "Camarero 105",
"Salesperson": true
}
]
}


## Categoría Formas de Pago

En esta categoría se incluirán todos los comandos que permitirán interactuar con datos de formas de pago. Los comandos
disponibles para esta categoría serán los siguientes:

1. **GetTenderList** : Obtiene las formas de pago dadas de alta en la base de datos.
2. **GetPOSTenderList** : Obtiene las formas de pago asociadas al número de terminal para el que se solicite.


### GetTenderList

Este comando devuelve una lista con todas las formas de pago configuradas en BDP-Net.

##### Ruta de llamada

/API/Tenders/GetList

##### Parámetros de entrada

Este comando no tiene parámetros de entrada.

##### JSON de solicitud

{
}

##### Parámetros de salida

**ErrorMessage** **_(string)_** : Cadena de errores. Si todo ha ido bien, estará vacía.

**TenderList** **_List (of Tender)_** : Lista de formas de pago obtenidas. Es un conjunto de estructuras del tipo **Tender**.

##### JSON de respuesta

{
"ErrorMessage": "",
"TenderList": [
{
"Id": 1,
"Name": "Efectivo",
"Type": 1
},...
{
"Id": 4,
"Name": "A 30 - 60 - 90",
"Type": 0
}
]
}


### etPOSTenderList

Este comando devuelve, de todas las formas de pago configuradas en la aplicación, una lista con las formas de pago que tiene
asignadas un terminal. Son las formas de pago que se usarían normalmente para cobrar una comanda desde el TPV (Efectivo, Tarjeta,
Pago con puntos de fidelización, etc.).

##### Ruta de llamada

/API/Tenders/GetPOSList

##### Parámetros de entrada

**POSId** **_(integer)_** : Código del terminal del que se van a obtener las formas de pago asociadas.

##### JSON de solicitud

{
"POSId": 1
}

##### Parámetros de salida

**ErrorMessage** **_(string)_** : Cadena de errores. Si todo ha ido bien, estará vacía.

**TenderList** **_List (of Tender)_** : Lista de formas de pago obtenidas. Es un conjunto de estructuras del tipo **Tender**.

##### JSON de respuesta

{
"ErrorMessage": "",
"TenderList": [
{
"Id": 1,
"Name": "Efectivo",
"Type": 1
},...
{
"Id": 3,
"Name": "Cheque",
"Type": 1
}
]
}


## Categoría Perfiles de Departamentos y Artículos

En esta categoría se incluirán todos los comandos que permitirán obtener los códigos numéricos de perfiles de departamentos y
artículos en los que se podrán añadir o modificar departamentos o artículos cuando estos estén siendo creados o modificados. Los
comandos disponibles para esta categoría serán los siguientes:

1. **GetProfilesListCreateDepartmentList** : Obtiene la lista de perfiles de departamentos y artículos disponibles en la base de
    datos donde se puede dar de alta un nuevo departamento.
2. **GetProfilesListCreateArticleList** : Obtiene la lista de perfiles de departamentos y artículos disponibles en la base de datos
    donde se puede dar de alta un nuevo artículo.
3. **GetProfileListModifyArticleList** : Obtiene la lista de perfiles de departamentos y artículos disponibles en la base de datos
    los cuales contienen el artículo especificado para que pueda ser modificado.


### GetProfilesListCreateDepartmentList

Este comando devuelve una lista de perfiles de departamentos y artículos en los que un departamento que esté siendo dado de
alta, puede ser añadido.

##### Ruta de llamada

/API/ProfilesLists/GetCreateDepartmentList

##### Parámetros de entrada

Este comando no tiene parámetros de entrada.

##### JSON de solicitud

{
}

##### Parámetros de salida

**ErrorMessage** (string): Cadena de errores. Si todo ha ido bien, estará vacía.

**ProfilesList** (Of **ProfilesListInfo** ): Colección de campos con información de los perfiles. **ProfilesListInfo** tiene la siguiente estructura:

```
Profile (integer): Número del perfil.
```
```
ProfileName (string): Descripción del perfil.
```
##### JSON de respuesta

{
"ErrorMessage": "",
"ProfilesList": [
{
"Profile": 1,
"ProfileName": "PERFIL 1"
}
]
}


### GetProfilesListCreateArticleList

Este comando devuelve una lista de perfiles de departamentos y artículos en los que un artículo que esté siendo dado de alta, puede
ser añadido.

##### Ruta de llamada

/API/ProfilesLists/GetCreateArticleList

##### Parámetros de entrada

**DeptoCode** (integer): Código del departamento a que pertenece el artículo.

##### JSON de solicitud

{
"DeptoCode": 1
}

##### Parámetros de salida

**ErrorMessage** (string): Cadena de errores. Si todo ha ido bien, estará vacía.

**ProfilesList** (Of **ProfilesListInfo** ): Colección de campos con información de los perfiles. A su vez, **ProfilesListInfo** Tiene la siguiente
estructura:

```
Profile (integer): Número del perfil.
```
```
ProfileName (string): Descripción del perfil.
```
##### JSON de respuesta

{
"ErrorMessage": "",
"ProfilesList": [
{
"Profile": 1,
"ProfileName": "PERFIL 1"
}
]
}


### GetProfilesListModifyArticleList

Este comando devuelve una lista de perfiles de departamentos y artículos en los que se puedan aplicar las modificaciones de artículo
de un artículo que vaya a ser modificado.

##### Ruta de llamada

/API/ProfilesLists/GetModifyArticleList

##### Parámetros de entrada

**ArtCode** (long): Código del artículo que se modifica.

##### JSON de solicitud

{
"ArtCode": 1001
}

##### Parámetros de salida

**ErrorMessage** (string): Cadena de errores. Si todo ha ido bien, estará vacía.

**ProfilesList** (Of **ProfilesListInfo** ): Colección de campos con información de los perfiles. **ProfilesListInfo** tiene la siguiente estructura:

```
Profile (integer): Número del perfil.
```
```
ProfileName (string): Descripción del perfil.
```
#### JSON de respuesta:

{
"ErrorMessage": "",
"ProfilesList": [
{
"Profile": 1,
"ProfileName": "PERFIL 1"
}
]
}


## Categoría Perfiles de Exportación

En esta categoría se incluirán todos los comandos que permitirán interactuar con los perfiles de exportación de la aplicación. Los
comandos disponibles para esta categoría serán los siguientes:

1. **ExportDocumentsByExportProfile** : Ejecuta la exportación de documentos de venta especificada devolviendo el resultado
    de la exportación.
2. **ExportStockAndSalesSummaryByExportProfile** : Ejecuta la exportación de movimientos de stock de ventas especificada
    devolviendo el resultado de la exportación.


### ExportDocumentsByExportProfile

Mediante este comando se podrá obtener una lista de documentos con una estructura definida por un perfil de exportación
existente en la aplicación. En la solicitud se deberá especificar el perfil de exportación a ser utilizado. En dicho perfil será en el que
se tendrán que escoger que campos van a ser exportados.

##### Ruta de llamada

/API/ExportProfiles/Documents

##### Parámetros de entrada

**ExportProfileCode (Integer):** Código de perfil de exportación de BDP (Utilidades -> Importación /Exportación -> Perfiles de
exportación).

**InitialDate (Date, Opcional, Por defecto: Date.Now.AddDays(-1)):** Fecha inicial de exportación.

**FinalDate (Date, Opcional, Por defecto: Date.Now.AddDays(1)):** Fecha final de exportación).

**OnlyLastMovements (Boolean, Opcional, Por defecto False):** Cada vez que se exportan documentos por WeblinkRestApi quedan
marcados como exportados, con esta variable se retornarán todos los documentos sin tener en cuenta esa marca (usando los
parámetros anteriores).

**Funcionamiento:**

A partir de la versión 33.2, en la exportación de Datos de Venta, si está marcado el check de “Ventas Detalladas”, se exportarán
también las sublíneas de: menús, fast-food, packs (con sus suplementos) y de suplementos.

Debe haber almenos marcado un campo en las líneas del documento de ventas a exportar.

##### JSON de solicitud

{
"ExportProfileCode": 1.0,
"InitialDate": "2020- 10 - 27T16:36:12.7780472+01:00",
"FinalDate": "2020- 10 - 29T16:36:12.7780472+01:00",
"OnlyLastMovements": true
}

##### Parámetros de salida

**DocumentsLists:** Colección de documentos incluidos en la exportación ejecutada.

**ErrorMessage** (string): Cadena de errores. Si todo ha ido bien, estará vacía.

##### JSON de respuesta

{
"DocumentsLists": null,
"ErrorMessage": "[403901]-PERFIL DE EXPORTACIÓN INCORRECTO"
}


### ExportStockAndSalesSummaryByExportProfile

Mediante este comando se podrá obtener una lista de movimientos de stock de ventas con una estructura definida por un perfil de
exportación existente en la aplicación. En la solicitud se deberá especificar el perfil de exportación a ser utilizado. En dicho perfil se
podrá especificar el rango de almacenes que deben estar incluidos en la exportación.

##### Ruta de llamada

/API/ExportProfiles/ StockAndSalesSummary

##### Parámetros de entrada

**ExportProfileCode (Integer):** Código de perfil de exportación de BDP (Utilidades -> Importación /Exportación -> Perfiles de
exportación).

**InitialDate (Date, Opcional, Por defecto: Date.Now.AddDays(-1)):** Fecha inicial de exportación.

**FinalDate (Date, Opcional, Por defecto: Date.Now.AddDays(1)):** Fecha final de exportación).

##### JSON de solicitud

{
"ExportProfileCode": 1.0,
"InitialDate": "2020- 10 - 27T16:36:12.7780472+01:00",
"FinalDate": "2020- 10 - 29T16:36:12.7780472+01:00",
}

##### Parámetros de salida

**ExportedData:** Lista de códigos de almacenes con movimientos de stock de artículos vendidos y descontados de dicho almacén. A
su vez, cada objeto Almacén estará compuesto de una lista de artículos con sus movimientos de stock de ventas. Cada objeto de
este tipo estará formado por los siguientes campos:

```
Almacén(decimal): Código de almacén del movimiento de stock de venta.
```
```
Artículo(decimal): Código de artículo correspondiente al movimiento de stock de venta.
```
```
D1(decimal): Valor sólo disponible en caso de trabajar con una aplicación de talla y color. Valor que indicará el identificador
de la dimensión 1 de talla y color correspondiente a este registro de movimiento de stock de ventas.
```
```
D2(decimal): Valor sólo disponible en caso de trabajar con una aplicación de talla y color. Valor que indicará el identificador
de la dimensión 2 de talla y color correspondiente a este registro de movimiento de stock de ventas.
```
```
D3(decimal): Valor sólo disponible en caso de trabajar con una aplicación de talla y color. Valor que indicará el identificador
de la dimensión 3 de talla y color correspondiente a este registro de movimiento de stock de ventas.
```
```
Uds(decimal): Cantidad de unidades del artículo en cuestión descontadas del almacén correspondiente debido a ventas
```
**ErrorMessage** (string): Cadena de errores. Si todo ha ido bien, estará vacía.

##### JSON de respuesta

{


```
"ExportedData": {
"1": [
{
"Almacen": 1.0,
"Articulo": 1001.0,
"D1": 1.0,
"D2": 2.0,
"D3": 1.0,
"Uds": 1.0
},...
{
```
```
}
],...
},
```
```
"Almacen": 1.0,
"Articulo": 1008.0,
"D1": 3.0,
"D2": 4.0,
"D3": 40,
"Uds": 1.0
```
"ErrorMessage": ""
}


### ExportManagmentDocumentsByExportProfile

Mediante este comando se podrá obtener una lista de albaranes y documentos de gestión con una estructura definida por un perfil
de exportación existente en la aplicación. En la solicitud se deberá especificar el perfil de exportación a ser utilizado. En dicho perfil
será en el que se tendrán que escoger qué campos van a ser exportados.

##### Ruta de llamada

/API/ExportProfiles/ManagmentDocuments

##### Parámetros de entrada

**ExportProfileCode (Integer):** Código de perfil de exportación de BDP (Utilidades -> Importación /Exportación -> Perfiles de
exportación).

**InitialDate (Date, Opcional, Por defecto: Date.Now.AddDays(-1)):** Fecha inicial de exportación.

**FinalDate (Date, Opcional, Por defecto: Date.Now.AddDays(1)):** Fecha final de exportación.

**OnlyLastMovements (Boolean, Opcional, Por defecto False):** Cada vez que se exportan documentos por WeblinkRestApi quedan
marcados como exportados, con esta variable se retornarán todos los documentos sin tener en cuenta esa marca (usando los
parámetros anteriores).

##### JSON de solicitud

{
"ExportProfileCode": 1.0,
"InitialDate": "2021- 01 - 01T16:36:12.7780472+01:00",
"FinalDate": "2021- 12 - 31T16:36:12.7780472+01:00",
"OnlyLastMovements": true
}

##### Parámetros de salida

**DocumentsLists:** Colección de documentos y albaranes de gestión incluidos en la exportación ejecutada.

**ErrorMessage (string):** Cadena de errores. Si todo ha ido bien, estará vacía.

##### JSON de respuesta

{
"DocumentsLists": {
"ALGes": {
"Cabecera": [
{
"Serie": "S1",
"Num_Albaran": 1.0,
"Codigo_Cliente": 1.0,
"Nombre_Cliente": "CLIENTE 1",
"Direccion1_Cliente": "C/ MAYOR 256",
"Direccion2_Cliente": "",
"CP_Cliente": "08080",
"Poblacion_Cliente": "BARCELONA",
"Provincia_Cliente": "BARCELONA",
"Fecha_Albaran": "2021- 07 - 27T00:00:00",


"Representante": 1.0,
"Albaran_NO_Valorado": false,
"Forma_Pago": 1.0,
"Cod_Iva1": 0.0,
"Porcen_Iva1": 10.0,
"Porcen_Rec1": 0.0,
"Base1": 16.39091,
"Impor_Iva1": 1.63909,
"Impor_Rec1": 0.0,
"Cod_Iva2": 0.0,
"Porcen_Iva2": 0.0,
"Porcen_Rec2": 0.0,
"Base2": 0.0,
"Impor_Iva2": 0.0,
"Impor_Rec2": 0.0,
"Cod_Iva3": 0.0,
"Porcen_Iva3": 0.0,
"Porcen_Rec3": 0.0,
"Base3": 0.0,
"Impor_Iva3": 0.0,
"Impor_Rec3": 0.0,
"Cod_Iva4": 0.0,
"Porcen_Iva4": 0.0,
"Porcen_Rec4": 0.0,
"Base4": 0.0,
"Impor_Iva4": 0.0,
"Impor_Rec4": 0.0,
"Cod_Iva5": 0.0,
"Porcen_Iva5": 0.0,
"Porcen_Rec5": 0.0,
"Base5": 0.0,
"Impor_Iva5": 0.0,
"Impor_Rec5": 0.0,
"Total_Bases": 16.39091,
"Total_Ivas": 1.63909,
"Total_Rec": 0.0,
"Total_Albaran": 18.03
},
{
"Serie": "S1",
"Num_Albaran": 2.0,
"Codigo_Cliente": 1.0,
"Nombre_Cliente": "CLIENTE 1",
"Direccion1_Cliente": "C/ MAYOR 256",
"Direccion2_Cliente": "",
"CP_Cliente": "08080",
"Poblacion_Cliente": "BARCELONA",
"Provincia_Cliente": "BARCELONA",
"Fecha_Albaran": "2021- 07 - 27T00:00:00",
"Representante": 1.0,
"Albaran_NO_Valorado": false,
"Forma_Pago": 1.0,
"Cod_Iva1": 0.0,
"Porcen_Iva1": 10.0,
"Porcen_Rec1": 0.0,
"Base1": 21.85455,
"Impor_Iva1": 2.18545,
"Impor_Rec1": 0.0,
"Cod_Iva2": 0.0,
"Porcen_Iva2": 0.0,
"Porcen_Rec2": 0.0,
"Base2": 0.0,
"Impor_Iva2": 0.0,
"Impor_Rec2": 0.0,
"Cod_Iva3": 0.0,
"Porcen_Iva3": 0.0,


"Porcen_Rec3": 0.0,
"Base3": 0.0,
"Impor_Iva3": 0.0,
"Impor_Rec3": 0.0,
"Cod_Iva4": 0.0,
"Porcen_Iva4": 0.0,
"Porcen_Rec4": 0.0,
"Base4": 0.0,
"Impor_Iva4": 0.0,
"Impor_Rec4": 0.0,
"Cod_Iva5": 0.0,
"Porcen_Iva5": 0.0,
"Porcen_Rec5": 0.0,
"Base5": 0.0,
"Impor_Iva5": 0.0,
"Impor_Rec5": 0.0,
"Total_Bases": 21.85455,
"Total_Ivas": 2.18545,
"Total_Rec": 0.0,
"Total_Albaran": 24.04
}
],
"Lineas": [
{
"Serie": "S1",
"Num_Albaran": 1.0,
"Linea": 1.0,
"Articulo": 1001.0,
"Descripcion": "CHULETÓN",
"Unidades": 1.0,
"Precio": 18.03,
"Dto1": 0.0,
"Dto2": 0.0,
"Total": 18.03,
"Porcen_Iva": 10.0,
"Porcen_Recargo": 0.0,
"Representante": 1.0,
"Almacen": 1.0,
"Fecha": "2021- 07 - 27T00:00:00",
"Precio_Coste_UPC": 0.0,
"Precio_Coste_PMC": 0.0
},
{
"Serie": "S1",
"Num_Albaran": 2.0,
"Linea": 1.0,
"Articulo": 1002.0,
"Descripcion": "EMBUTIDOS",
"Unidades": 2.0,
"Precio": 12.02,
"Dto1": 0.0,
"Dto2": 0.0,
"Total": 24.04,
"Porcen_Iva": 10.0,
"Porcen_Recargo": 0.0,
"Representante": 1.0,
"Almacen": 1.0,
"Fecha": "2021- 07 - 27T00:00:00",
"Precio_Coste_UPC": 0.0,
"Precio_Coste_PMC": 0.0
}
]
},
"FAGes": {
"Cabecera": [
{


"Serie": "S1",
"Num_Factura": 1.0,
"Fecha_Factura": "2021- 07 - 27T00:00:00",
"Forma_Pago": 1.0,
"Porcen_Iva1": 10.0,
"Porcen_Rec1": 0.0,
"Base1": 21.85455,
"Impor_Iva1": 2.18545,
"Impor_Rec1": 0.0,
"Porcen_Iva2": 0.0,
"Porcen_Rec2": 0.0,
"Base2": 0.0,
"Impor_Iva2": 0.0,
"Impor_Rec2": 0.0,
"Porcen_Iva3": 0.0,
"Porcen_Rec3": 0.0,
"Base3": 0.0,
"Impor_Iva3": 0.0,
"Impor_Rec3": 0.0,
"Porcen_Iva4": 0.0,
"Porcen_Rec4": 0.0,
"Base4": 0.0,
"Impor_Iva4": 0.0,
"Impor_Rec4": 0.0,
"Porcen_Iva5": 0.0,
"Porcen_Rec5": 0.0,
"Base5": 0.0,
"Impor_Iva5": 0.0,
"Impor_Rec5": 0.0,
"Total_Bases": 21.85455,
"Total_Ivas": 2.18545,
"Total_Rec": 0.0,
"Total_Factura": 24.04,
"Codigo_Cliente": 1.0,
"Nombre_Fiscal": "CLIENTE 1",
"Nombre_Comercial": "CLIENTE 1",
"Nif_Cliente": "",
"Direccion1_Cliente": "C/ MAYOR 256",
"Direccion2_Cliente": "",
"CP_Cliente": "08080",
"Poblacion_Cliente": "BARCELONA",
"Provincia_Cliente": "BARCELONA",
"Telefono1": "",
"Telefono2": "",
"TelefonoMovil": "",
"Fax": "",
"E_Mail": "",
"Porcen_Dto": 0.0,
"Importe_Dto": 0.0,
"Porcen_Reten_Fiscal": 0.0,
"Base_Reten_Fiscal": 0.0,
"Importe_Reten_Fiscal": 0.0,
"Porcen_Reten_Comercial": 0.0,
"Base_Reten_Comercial": 0.0,
"Importe_Reten_Comercial": 0.0,
"Total_A_Liquidar": 24.04,
"Porcen_Dto_ProntoPago": 0.0,
"Base_Dto_ProntoPago": 0.0,
"Importe_Dto_ProntoPago": 0.0,
"Porcen_Financiacion": 0.0,
"Base_Financiacion": 0.0,
"Importe_Financiacion": 0.0,
"Inicio_Cta_IBAN": "",
"Cuenta_Corriente": "",
"Tipo_Remesa": 1.0,
"Es_FraRectificada": false,


"Fra_Rectidicada_Serie": "",
"Fra_Rectificada_NumFactura": 0.0,
"Fra_Rectificada_Fecha": "1900- 01 - 01T00:00:00",
"Fra_Rectificada_Importe": 0.0,
"Fra_Rectificada_Concepto1": "",
"Fra_Rectificada_Concepto2": "",
"Fra_Rectificada_concepto3": ""
}
],
"Lineas": [
{
"Serie": "S1",
"Num_Factura": 1.0,
"Linea": 1.0,
"Serie_Albaran": "S1",
"Num_Albaran": 2.0,
"Fecha_Albaran": "2021- 07 - 27T00:00:00",
"Total_Bases": 21.85455,
"Total_Ivas": 2.18545,
"Total_Recargos": 0.0,
"Total": 24.04,
"Base1": 21.85455,
"Base2": 0.0,
"Base3": 0.0,
"Base4": 0.0,
"Base5": 0.0,
"Porcen_Iva1": 10.0,
"Porcen_Iva2": 0.0,
"Porcen_Iva3": 0.0,
"Porcen_Iva4": 0.0,
"Porcen_Iva5": 0.0,
"Impor_Iva1": 2.18545,
"Impor_Iva2": 0.0,
"Impor_Iva3": 0.0,
"Impor_Iva4": 0.0,
"Impor_Iva5": 0.0,
"Porcen_Rec1": 0.0,
"Porcen_Rec2": 0.0,
"Porcen_Rec3": 0.0,
"Porcen_Rec4": 0.0,
"Porcen_Rec5": 0.0,
"Impor_Rec1": 0.0,
"Impor_Rec2": 0.0,
"Impor_Rec3": 0.0,
"Impor_Rec4": 0.0,
"Impor_Rec5": 0.0,
"Total_1": 24.04,
"Total_2": 0.0,
"Total_3": 0.0,
"Total_4": 0.0,
"Total_5": 0.0
}
],
"Vencimientos": [
{
"Serie": "S1",
"Num_Factura": 1.0,
"Linea": 1.0,
"Fecha": "2021- 07 - 27T00:00:00",
"Importe": 24.04,
"Recibo": false,
"Medio_De_Cobro": 0.0,
"Num_Remesa": 0.0,
"Pagado": false,
"Fecha_Pago": "1900- 01 - 01T00:00:00",
"Devolucion": false,


"Comentario_Devolucion": "",
"Cobro_Desde_Tpv": false,
"Forma_Pago": 0.0,
"Terminal": 0.0,
"Cuenta_Bancaria": ""
}
]
}
},
"ErrorMessage": ""
}


### ExportPurchaseNotes

Mediante este comando se podrá obtener una lista de albaranes de compra con una estructura definida por un perfil de exportación
existente en la aplicación. En la solicitud se deberá especificar el perfil de exportación a ser utilizado. En dicho perfil será en el que
se tendrán que escoger qué campos van a ser exportados.

##### Ruta de llamada

/API/ExportProfiles/PurchaseNotes

##### Parámetros de entrada

**ExportProfileCode (Integer):** Código de perfil de exportación de BDP (Utilidades -> Importación /Exportación -> Perfiles de
exportación).

**InitialDate (Date, Opcional, Por defecto: Date.Now):** Fecha inicial de exportación.

**FinalDate (Date, Opcional, Por defecto: Date.Now):** Fecha final de exportación.

**InitialSuplier (Long, Por defecto: 0):** Proveedor inicial de la exportación

**FinalSuplier (Long, Por defecto: 999999999):** Proveedor final de la exportación

**InitialSerial (String, Por defecto: “”** ➔ **cadena vacía):** Serie inicial de la exportación

**FinalSerial (String, Por defecto: zzzzzz):** Serie final de la exportación

##### JSON de solicitud

{
"ExportProfileCode": 1.0,
"InitialDate": "2022- 10 - 01T00:00:00",
"FinalDate": "2022- 11 - 07T00:00:00",
"InitialSupplier": 1,
"FinalSupplier": 2,
"InitialSerial": "",
"FinalSerial": "zzzzzz"
}

##### Parámetros de salida

**DocumentsLists:** Colección de albaranes de compra incluidos en la exportación ejecutada.

**ErrorMessage (string):** Cadena de errores. Si todo ha ido bien, estará vacía.

##### JSON de respuesta

{
"ExportedData": {
"Cabecera": [
{


"Serie_Albaran": "S1",
"Codigo": 1.0,
"Cod_Proveedor": 1.0,
"Nom_Proveedor": "BEBIDAS, S.L.",
"Fecha_Albaran": "2022- 11 - 07T00:00:00",
"Total_IVA": 5.46,
"Total_RE": 0.0,
"Total_Albaran": 31.46
},
{
"Serie_Albaran": "S1",
"Codigo": 2.0,
"Cod_Proveedor": 2.0,
"Nom_Proveedor": "COMIDAS, S.A.",
"Fecha_Albaran": "2022- 11 - 07T00:00:00",
"Total_IVA": 31.5,
"Total_RE": 0.0,
"Total_Albaran": 181.5
}
],
"Lineas": [
{
"Serie_Albaran": "S1",
"Codigo": 1.0,
"Linea": 1.0,
"Cod_Articulo": 1001.0,
"Descripcion_Articulo": "COCA-COLA",
"Uds": 2.0,
"Precio": 3.0,
"Dto": 0.0,
"Total": 6.0
},
{
"Serie_Albaran": "S1",
"Codigo": 1.0,
"Linea": 2.0,
"Cod_Articulo": 1002.0,
"Descripcion_Articulo": "SCHWEPPES LIMÓN",
"Uds": 5.0,
"Precio": 4.0,
"Dto": 0.0,
"Total": 20.0
},
{
"Serie_Albaran": "S1",
"Codigo": 2.0,
"Linea": 1.0,
"Cod_Articulo": 1003.0,
"Descripcion_Articulo": "SCHWEPPES NARANJA",
"Uds": 100.0,
"Precio": 1.5,
"Dto": 0.0,
"Total": 150.0
}
]
},
"ErrorMessage": ""
}

**NOTA:** Se han seleccionado solo algunos campos de las cabeceras y de las líneas de albaranes de compra.


## Categoría Stock

En esta categoría se incluirán todos los comandos que permitirán interactuar con daos del apartado de stock. Los comandos
disponibles para esta categoría serán los siguientes:

1. **CreateFamily** : Crea una familia en la base de datos.
2. **CreateSubfamily** : Crea una subfamilia en la base de datos.
3. **GetStock** : Obtiene el stock de un artículo.
4. **GetListStock** : Crea una lista de pares “Código Artículo”, ”Stock Actual”.
5. **GetItemCostPrices** : Obtiene los precios de coste del artículo especificado.
6. **GetItemsCostPrices** : Obtiene una lista de precios de coste para una lista de artículos.
7. **Regularizations** : Crea una regularización de stock para el artículo especificado.
8. **Transfers** : Crea un traspaso entre almacenes para el artículo especificado.
9. **UpdateMassiveStock** : Crea una regularización para los artículos especificados con la que se incrementará/decrementará
    el stock de los artículos en el valor especificado.
10. **UpdateStock** : Crea una regularización para el artículo especificado con la que se incrementará/decrementará el stock del
    artículo en el valor especificado.
11. **UpdateMasiveInventory** : Crea una comprobación de stock para los artículos especificados con la que se dejará el stock de
    los artículos en el valor especificado.


### CreateFamily

Mediante este comando se creará una familia con el código y la descripción facilitadas en los parámetros de entrada. Si ya existiese
la familia, si **Overwrite** =True, sobrescribe la familia; en caso contrario, no lo hace y emite un error informando de la existencia de la
familia. Si la descripción de la familia excede los 40 caracteres, la rutina recortará la descripción a los primeros 40 caracteres. Cuando
se está creando una nueva familia, el programa no puede estar trabajando con el mantenimiento de familias. Si es así, se generará
un error de bloqueo y se informará de ello.

##### Ruta de llamada

/API/Warehouse/CreateFamily

##### Parámetros de entrada

**Code** **_(integer)_** : Número entero de 1 a 3 dígitos mayor que cero. Corresponde al código de la familia que se desea crear.

**Description** **_(string)_** : Cadena de texto no vacía con un máximo de 40 caracteres. Corresponde a la descripción o nombre de la familia
que se desea crear.

**Overwrite** **_(boolean)_** : Campo booleano (True: Sí; False: No) que indica si deseamos sobrescribir una familia en el caso de que ya
exista en la base de datos.

##### JSON de solicitud

{
"Code": 55,
"Description": "FAMILIA WEBLINK",
"Overwrite": true
}

##### Parámetros de salida

**ErrorMessage** **_(string)_** : Cadena de errores. Si todo ha ido bien, estará vacía.

##### JSON de respuesta

{
"ErrorMessage": ""
}


### CreateSubFamily

Mediante este comando se creará una subfamilia con el código y la descripción facilitadas en los parámetros de entrada. Si ya
existiese la subfamilia, si **Overwrite** =True, sobrescribe la subfamilia; en caso contrario, no lo hace y emite un error informando de la
existencia de la subfamilia. Si la descripción de la subfamilia excede los 40 caracteres, la rutina recortará la descripción a los primeros
40 caracteres. Cuando se está creando una nueva subfamilia, el programa no puede estar trabajando con el mantenimiento de
subfamilias. Si es así, se generará un error de bloqueo y se informará de ello.

##### Ruta de llamada

/API/Warehouse/CreateSubfamily

##### Parámetros de entrada

**Code** **_(integer)_** : Número entero de 1 a 3 dígitos mayor que cero. Corresponde al código de la subfamilia que se desea crear.

**Description** **_(string)_** : Cadena de texto no vacía con un máximo de 40 caracteres. Corresponde a la descripción o nombre de la
subfamilia que se desea crear.

**Overwrite** **_(boolean)_** : Campo booleano (True: Sí; False: No) que indica si deseamos sobrescribir una subfamilia en el caso de que ya
exista en la base de datos.

##### JSON de solicitud

{
"Code": 88,
"Description": "SUBFAMILIA WEBLINK",
"Overwrite": true
}

##### Parámetros de salida

**ErrorMessage** **_(string)_** : Cadena de errores. Si todo ha ido bien, estará vacía.

##### JSON de respuesta

{
"ErrorMessage": ""
}


### GetStock

Mediante este comando se podrá obtener el stock de un artículo. Si el artículo no es de talla y color, la rutina devuelve el stock del
artículo y el almacén especificados. Si no existe movimiento alguno, devuelve cero. Si el artículo es de talla y color, la rutina devuelve
el stock del artículo y almacén especificados para su correspondiente talla y color, que viene determinada por el Codigo alternativo
autogenerado. Si hay discrepancias entre el código principal y el código autogenerado, se producirá un mensaje de error.

##### Ruta de llamada

/API/Warehouse/GetStock

##### Parámetros de entrada

**Article** **_(long)_** : Número entero de 1 a 13 dígitos mayor que cero. Código principal del artículo, que debe ser de tipo Web.

**Altern** **_(integer)_** : Código alternativo autogenerado por BDP del artículo (si es Talla y Color; si no, valdrá cero).

**Store** **_(integer)_** : Número entero de 1 a 6 dígitos mayor que cero. Corresponde al código de almacén.

##### JSON de solicitud

{
"Article": 1001,
"Altern": 0,
"Store": 1
}

##### Parámetros de salida

**Stock** **_(decimal)_** : Valor decimal con el stock correspondiente. Si se ha producido un error, el stock será devuelto a cero.

**ErrorMessage** **_(string)_** : Cadena de errores. Si todo ha ido bien, estará vacía.

##### JSON de respuesta

{
"Stock": 0.0,
"ErrorMessage": ""
}


### GetListStock

Esta rutina es análoga a GetStock, sólo que en vez de devolver un solo valor de stock, devuelve un valor de stock para cada artículo
especificado. Si el artículo no es de talla y color, la rutina devuelve el stock del artículo y el almacén especificados. Si no existe
movimiento alguno, devuelve cero. Si el artículo es de talla y color, la rutina devuelve el stock del artículo y almacén especificados
para su correspondiente talla y color, que viene determinada por el Código alternativo autogenerado. Si hay discrepancias entre el
código principal y el código autogenerado, se producirá un mensaje de error.

##### Ruta de llamada

/API/Warehouse/GetListStock

##### Parámetros de entrada

**Articles** **_List(of ArticleListItem)_** : Colección de artículos del tipo **ArticleListItem**. A su vez, ListArticles tiene la siguiente estructura:

```
Article (long) : Número entero de 1 a 13 dígitos mayor que cero. Corresponde al código principal del artículo. Cada artículo
debe ser del tipo Web.
```
```
Altern (integer) : Código alternativo autogenerado por BDP del artículo (si es Talla y Color; si no, valdrá cero).
```
**Store** **_(integer)_** : Número entero de 1 a 6 dígitos mayor que cero. Corresponde al código de almacén.

##### JSON de solicitud

{
"Store": 1,
"Articles": [
{
"Article": 1001,
"Altern": 0
},...
{
"Article": 1005,
"Altern": 0
}
]
}

##### Parámetros de salida

**Stock** **_List(of ListStock)_** : Colección de valores del stock del tipo ListStock. A su vez, ListStock tiene la siguiente estructura:

```
Article (long) : Número entero de 1 a 13 dígitos mayor que cero. Corresponde al código principal del artículo.
```
```
Altern (integer) : Código alternativo autogenerado por BDP del artículo (si es Talla y Color; si no, valdrá cero).
```
```
Units (decimal) : Valor del stock para este artículo. Si se ha producido un error, valdrá cero.
```
```
ErrorMessage (string) : Cadena de errores. Si todo ha ido bien, estará vacía.
```
**ErrorMessage** **_(string)_** : Cadena de errores. Si todo ha ido bien, estará vacía. Si se ha producido algún error en una línea de ListStock,
aquí se indicará la existencia de errores.


##### JSON de respuesta

{
"Stock": [
{
"Article": 1001,
"Altern": 0,
"Units": 0.0,
"ErrorMessage": ""
},...
{
"Article": 1005,
"Altern": 0,
"Units": 0.0,
"ErrorMessage": ""
}
],
"ErrorMessage": ""
}


### GetItemCostPrices

Rutina pensada para obtener el ultimo precio de coste (UPM) y el precio medio de coste (PMC) de un artículo dado

##### Ruta de llamada

/API/Warehouse/GetItemCostPrices

##### Parámetros de entrada

**Article:** Artículo que querermos solicitar.

##### JSON de solicitud

{
"Article": 1001
}

##### Parámetros de salida

**UPC (Decimal)** : Último precio de compra.

**PMC (Decimal)** : Precio medio de compra

**ErrorMessage** **_(string)_** : Cadena de errores. Si todo ha ido bien, estará vacía. Si se ha producido algún error en una línea de ListStock,
aquí se indicará la existencia de errores.

##### JSON de respuesta

{
"UPC": 0,
"PMC": 0,
"ErrorMessage": ""
}


### GetItemsCostPrices

Esta función ejecuta para una lista de artículos el comando “GetItemCostPrices”, se ha programado con la idea de reducir llamadas
al TPV.

##### Ruta de llamada

/API/Warehouse/GetItemsCostPrices

##### Parámetros de entrada

**Artifcles** **_(List(Of Long)_** **)** : Lista de códigos de artículos de los que queremos obtener los precios de compra

##### JSON de solicitud

{
"Articles": [1001,1002,1003]
}

##### Parámetros de salida

**ErrorMessage** **_(string):_** Cadena de errores. Si todo ha ido bien, estará vacía.

**_Results Dictionary(of string, wsOutputGetItemCostPrice):_** Diccionario indexado por código de artículo, cada valor contiene la salida
que retornaría el comando GetItemCostPrices.

##### JSON de respuesta

{
"Results": {
"1001": {
"PMC": 0.0,
"UPC": 0.0,
"ErrorMessage": ""
},
"1002": {
"PMC": 0.0,
"UPC": 0.0,
"ErrorMessage": ""
},
"1003": {
"PMC": 0.0,
"UPC": 0.0,
"ErrorMessage": ""
}
},
"ErrorMessage": ""
}


### Regularizations

Se generará la regularización de un artículo y unas unidades, para un almacén y con un motivo de regularización facilitado. Artículo,
almacén y motivo de regularización deberán existir en la base de datos. Si el artículo es escandallado, se regularizará el escandallo.

##### Ruta de llamada

/API/Warehouse/Regularizations

##### Parámetros de entrada

**Article(** **_long_** **)** : Número entero entre 1 y 9999999999999. Corresponde al código principal del artículo a regularizar.

**sD1(** **_string_** **)** : Cadena de texto con un máximo de 10 caracteres (puede estar vacía) que indica el nombre de la primera dimensión del
artículo, en el caso de que sea de talla y color. Si no es de talla y color, debe dejarse vacía.

**sD2(** **_string_** **)** : Cadena de texto con un máximo de 10 caracteres (puede estar vacía) que indica el nombre de la segunda dimensión del
artículo, en el caso de que sea de talla y color. Si no es de talla y color, debe dejarse vacía.

**sD3(** **_string_** **)** : Cadena de texto con un máximo de 10 caracteres (puede estar vacía) que indica el nombre de la tercera dimensión del
artículo, en el caso de que sea de talla y color. Si no es de talla y color, debe dejarse vacía.

**Units(** **_decimal_** **)** : Valor entero que indica las unidades que se regularizan. Puede ser positivo o negativo.

**CodReg(** **_integer_** **)** : Valor entero entre 1 y 99 que indica el motivo de regularizaciones.

**Store(** **_integer_** **)** : Valor entero entre 1 y 999999 que indica el número de almacén al que se aplicará la regularización.

**DateReg(** **_date_** **)** : Fecha en que se efectúa la regularización, en formato dd/mm/aaaa.

##### JSON de solicitud

{
"Article": 1001,
"sD1": "",
"sD2": "",
"sD3": "",
"Units": 9.0,
"CodReg": 1,
"Store": 1,
"DateReg": "2020- 10 - 29T11:45:19.7568576+01:00"
}

##### Parámetros de salida

**ErrorMessage** (string): Cadena de errores. Si todo ha ido bien, estará vacía. Contiene errores generales de validación.

##### JSON de respuesta

{
"ErrorMessage": ""
}


### Transfers

Se generará un traspaso de un artículo entre un almacén de salida y un almacén de entrada de un artículo, con un motivo de traspaso
facilitado. Artículo, almacenes y motivo de regularización deberán existir en la base de datos. El artículo debe ser inventariable.

##### Ruta de llamada

/API/Warehouse/Transfers

##### Parámetros de entrada

**Article(** **_long_** **)** : Número entero entre 1 y 9999999999999. Corresponde al código principal del artículo a traspasar.

**sD1(** **_string_** **)** : Cadena de texto con un máximo de 10 caracteres (puede estar vacía) que indica el nombre de la primera dimensión del
artículo, en el caso de que sea de talla y color. Si no es de talla y color, debe dejarse vacía.

**sD2(** **_string_** **)** : Cadena de texto con un máximo de 10 caracteres (puede estar vacía) que indica el nombre de la segunda dimensión del
artículo, en el caso de que sea de talla y color. Si no es de talla y color, debe dejarse vacía.

**sD3(** **_string_** **)** : Cadena de texto con un máximo de 10 caracteres (puede estar vacía) que indica el nombre de la tercera dimensión del
artículo, en el caso de que sea de talla y color. Si no es de talla y color, debe dejarse vacía.

**Units(** **_decimal_** **)** : Valor entero que indica las unidades que se traspasan. Puede ser positivo o negativo.

**CodTransfer(** **_integer_** **)** : Valor entero entre 1 y 99 que indica el motivo de traspaso.

**StoreFrom(** **_integer_** **)** : Valor entero entre 1 y 999999 que indica el número de almacén de salida del traspaso.

**StoreTo(** **_integer_** **)** : Valor entero entre 1 y 999999 que indica el número de almacén de entrada del traspaso.

**DateTransfer(** **_date_** **)** : Fecha en que se efectúa el traspaso entre almacenes, en formato dd/mm/aaaa.

##### JSON de solicitud

{
"Article": 1001,
"sD1": "",
"sD2": "",
"sD3": "",
"Units": 6.0,
"CodTransfer": 1,
"StoreFrom": 1,
"StoreTo": 2,
"DateTransfer": "2020- 10 - 29T18:01:44.8587677+01:00"
}

##### Parámetros de salida

**ErrorMessage** (string): Cadena de errores. Si todo ha ido bien, estará vacía. Contiene errores generales de validación.

##### JSON de respuesta

{
"ErrorMessage": ""


}


### UpdateMassiveStock

Mediante este comando se podrá generar una regularización de stock según los parámetros de entrada especificados. Si las unidades
son positivas, suma stock y si son negativas resta stock. En las aplicaciones de hostelería, si el artículo es no inventariable y
escandallado, incluye en la regularización de manera automática su escandallo.

##### Ruta de llamada

/API/Warehouse/UpdateMassiveStock

##### Parámetros de entrada

**CodReg** **_(integer)_** : Número entero de 1 ó 2 dígitos. Corresponde al código del motivo de traspaso que se aplica a la regularización.

**Store** **_(integer)_** : Número entero de 1 a 6 dígitos que corresponde al código de almacén del que se descontará el stock.

**DateReg** **_(datetime)_** : Fecha en que se efectúa la regularización.

**ArticlesList** **_(InfoStock)_** : Datos de los artículos sobre los que se quiere regularizar el stock. Es una estructura del tipo **InfoStock** , que
tiene los siguientes elementos:

```
Article (long) : Código del artículo. Número entero de 1 a 13 dígitos mayor que cero. Corresponde al código principal del
artículo del que se va a regularizar el stock. El artículo debe ser del tipo Web y no ser de Talla y Color.
```
```
Units (decimal) : Unidades que se regularizan. Pueden ser positivas o negativas.
```
##### JSON de solicitud

{
"CodReg": 1,
"Store": 1,
"DateReg": "2020- 10 - 29T18:22:09.9509839+01:00",
"ArticlesList": [
{
"Article": 1001,
"Units": 3.0
},...
{
"Article": 1003,
"Units": 9.0
}
]
}

##### Parámetros de salida

**ErrorMessage** **_(string)_** : Cadena de errores. Si todo ha ido bien, estará vacía. Corresponde a los errores generales de validación, como
fechas de regularización, almacenes o motivos de regularización.

**ErrorList** **_(ArticleError)_** : Datos sobre los errores de validación de los artículos. Es una estructura del tipo **ArticleError** , que tiene los
siguientes elementos:

```
Article (long) : Código del artículo.
```
```
ErrorMessage (string) : Cadena de texto que muestra los errores de validación del artículo. Si no, devuelve una cadena vacía.
```

##### JSON de respuesta

{
"ErrorMessage": "",
"ErrorList": [
{
"Article": 1001,
"ErrorMessage": ""
},...
{
"Article": 1003,
"ErrorMessage": "EL ARTÍCULO 1003 NO ES DEL TIPO WEB"
}
]
}


### UpdateStock

Mediante este comando se podrá generar una regularización de stock según los parámetros de entrada especificados. Si las unidades
son positivas, suma stock y si son negativas resta stock. En las aplicaciones de hostelería, si el artículo es no inventariable y
escandallado, incluye en la regularización de manera automática su escandallo. Si el artículo es de talla y color, genera la
regularización correspondiente a la talla/color/copa especificada.

##### Ruta de llamada

/API/Warehouse/UpdateStock

##### Parámetros de entrada

**Article** **_(long)_** : Número entero de 1 a 13 dígitos mayor que cero. Corresponde al código principal del artículo del que se va a actualizar
el stock. El artículo debe ser del tipo Web.

**Altern** **_(integer)_** : Código alternativo autogenerado por BDP del artículo (si es Talla y Color; si no, valdrá cero).

**Units** **_(decimal)_** : Unidades que se regularizan. Pueden ser positivas o negativas.

**CodReg** **_(integer)_** : Número entero de 1 ó 2 dígitos. Corresponde al código del motivo de traspaso que se aplica a la regularización.

**Store** **_(integer)_** : Número entero de 1 a 6 dígitos que corresponde al código de almacén del que se descontará el stock.

**DateReg** **_(datetime)_** : Fecha en que se efectúa la regularización.

##### JSON de solicitud

{
"Article": 1001,
"Altern": 0,
"Units": - 5.0,
"CodReg": 1,
"Store": 1,
"DateReg": "2020- 10 - 29T18:31:27.1592351+01:00"
}

##### Parámetros de salida

**ErrorMessage** **_(string)_** : Cadena de errores. Si todo ha ido bien, estará vacía.

##### JSON de respuesta

{
"ErrorMessage": ""
}


### UpdateMassiveInventory

Mediante este comando se podrá generar regularizaciones a partir de una comprobación de inventario según los parámetros de
entrada especificados. Por ejemplo, si el stock que calcula el programa es 10 y nosotros le decimos que realmente es 15, generará
una regularización de 15 - 10=+5 unidades. Si el stock del programa es 10 y le decimos que realmente es 5, generará una regularización
de 5-10=-5 unidades. Si el stock del programa es 10 y le decimos que realmente es 10, no generará nada (10-10=0): la
regularizaciones nulas no se generan.

##### Ruta de llamada

/API/Warehouse/UpdateMassiveInventory

##### Parámetros de entrada

**CodReg** **_(integer)_** : número entero de 1 ó 2 dígitos. Corresponde al código del motivo de traspaso que se aplica a la regularización de
inventario.

**Store** **_(integer)_** : número entero de 1 a 6 dígitos que corresponde al código de almacén al que se aplicará la regularización de
inventario.

**DateReg** **_(datetime)_** : fecha en que se efectúa la regularización de inventario.

**ArticlesList** **_(InfoStock)_** : datos de los artículos sobre los que se quiere regularizar el inventario. Es una estructura del tipo **InfoStock** ,
que tiene los siguientes elementos:

```
Article (long) : código del artículo. Número entero de 1 a 13 dígitos mayor que cero. Corresponde al código principal del
artículo del que se va a regularizar el inventario. El artículo debe ser del tipo Web, Inventariable y no ser de Talla y Color.
```
```
Units (decimal) : unidades que se regularizan de inventario. Pueden ser positivas o negativas.
```
##### JSON de solicitud

{
"CodReg": 1,
"Store": 1,
"DateReg": "2020- 10 - 29T18:40:43.3830954+01:00",
"ArticlesList": [
{
"Article": 1001,
"Units": 25.0
},...
{
"Article": 1003,
"Units": - 10.0
}
]
}

##### Parámetros de salida

**ErrorMessage** **_(string)_** : Cadena de errores. Si todo ha ido bien, estará vacía.


##### JSON de respuesta

{
"ErrorMessage": "",
"ErrorList": [
{
"Article": 1001,
"ErrorMessage": ""
},...
{
"Article": 1003,
"ErrorMessage": "EL ARTÍCULO 1003 NO ES DEL TIPO WEB"
}
]
}


## Categoría Suplementos

En esta categoría se incluirán todos los comandos que permitirán interactuar con datos de suplementos. Los comandos disponibles
para esta categoría serán los siguientes:

1. **GetSupplementsProfiles** : Obtiene una lista de artículos pertenecientes a un perfil de suplementos, y además solapa la
    información del artículo que se encuentra en el perfil de departamentos y artículos solicitado. Este comando está
    pensado para montar un e-Commerce ya que incorpora información como por ejemplo los Alergenos y sólo incorpora la
    información que puede gestionar la comanda.
2. **GetPOSSupplementsProfile** : Obtiene una lista de artículos pertenecientes a un perfil de suplementos, y además solapa la
    información del artículo que se encuentra en el perfil de departamentos y artículos solicitado. Este comando está
    pensado para mostrar la información tal y como la está mostrando la pantalla de ventas (Los datos extraídos són los datos
    c)
3. **GetPOSSupplementsProfile** : Obtiene una lista de artículos pertenecientes a un perfil de suplementos, y además solapa la
    información del artículo que se encuentra en el perfil de departamentos y artículos solicitado.


### GetSupplementsProfile

Mediante este comando se podrá obtener la información de un perfil de suplementos. Cada suplemento incorpora la información
de su artículo correspondiente. Si el artículo se encuentra en el perfil de artículos solicitado, se sobrescribe con la información que
se encuentra en dicho perfil. Este comando está pensado para montar un e-Commerce ya que incorpora información como por
ejemplo los Alergenos y sólo incorpora la información que puede gestionar la comanda.

##### Ruta de llamada

/API/SupplementsProfiles/GetProfile

##### Parámetros de entrada

**SupplementsProfileID:** Código de perfil de suplementos que queremos consultar.

**ArticlesProfileID:** Código de perfil de artículos de donde vamos a sacar la información de los suplementos.

##### JSON de solicitud

{
"SupplementsProfileID": 1,
"ArticlesProfileID": 1
}

##### Parámetros de salida

**ErrorMessage** **_(string)_** : Descripción del posible error.

**SupplementsProfiles** **_List (of SupplementsProfile)_** : Lista de perfiles de suplementos obtenidos. Es un conjunto de estructuras del
tipo **SupplementsProfile** , que tiene los siguientes elementos:

```
Id (integer) : código del perfil de suplementos.
```
```
Name (string) : Descripción del perfil de suplementos.
```
```
Supplements List(of Supplement) : Lista de suplementos del perfil. Es un conjunto de estructuras del tipo Supplement , que
tiene los siguientes elementos:
```
#### Alergenos( AlergenosDataType ) : Lista de objetos AlergenosDataType que contendrá los datos de los alérgenos

```
asociados al artículo en cuestión. Cada objeto AlergenosDataType corresponderá a un alérgeno y estará
compuesto por los siguientes elementos:
```
```
Código: Valor numérico correspondiente al código del alérgeno asociado al artículo.
```
```
Descripción: Cadena con la descripción del alérgeno asociado al artículo.
```
```
ArtCode (Long) : Código del artículo.
```
```
ArtDescription (String) : Descripción del artículo.
```
```
Price1 (Decimal ): Precio de la tarifa 1.
```
```
Price2 (Decimal ): Precio de la tarifa 2.
```

```
Price3 (Decimal ): Precio de la tarifa 3.
```
```
Price4 (Decimal ): Precio de la tarifa 4.
```
```
Price5 (Decimal ): Precio de la tarifa 5.
```
```
Dct1 (Decimal): Descuento de la tarifa 1.
```
```
Dct2 (Decimal): Descuento de la tarifa 2.
```
```
Dct3 (Decimal): Descuento de la tarifa 3.
```
```
Dct4 (Decimal): Descuento de la tarifa 4.
```
```
Dct5 (Decimal): Descuento de la tarifa 5.
```
```
GraphDescrip1: Descripción Grafica Línea 1.
```
```
GraphDescrip2: Descripción Grafica Línea 2.
```
```
GraphDescrip3: Descripción Grafica Línea 3.
```
```
ExtendedArtDescription (string) : Cadena de texto que corresponde a la Descripción Extendida del artículo.
```
##### JSON de respuesta

{
"ErrorMessage": "",
"SupplementsProfile": {
"Id": 1.0,
"Name": "Perfil Suplementos 1",
"Supplements": [
{
"Alergenos": [],
"ArtDescription": "COCA-COLA",
"Dct1": 0.0,
"Dct2": 0.0,
"Dct3": 0.0,
"Dct4": 0.0,
"Dct5": 0.0,
"ExtendedArtDescription": "",
"GraphDescrip1": "",
"GraphDescrip2": "",
"GraphDescrip3": "",
"Price1": 1.05,
"Price2": 1.05,
"Price3": 1.05,
"Price4": 1.05,
"Price5": 1.05,
"ArtCode": 1001
},...
{
"Alergenos": [],
"ArtDescription": "SCHWEPPES NARANJA",
"Dct1": 0.0,
"Dct2": 0.0,
"Dct3": 0.0,
"Dct4": 0.0,
"Dct5": 0.0,
"ExtendedArtDescription": "",
"GraphDescrip1": "",


"GraphDescrip2": "",
"GraphDescrip3": "",
"Price1": 1.05,
"Price2": 1.05,
"Price3": 1.05,
"Price4": 1.05,
"Price5": 1.05,
"ArtCode": 1003
}
]
}
}


### GetPOSSupplementsProfile

Mediante este comando se podrá obtener la información de un perfil de suplementos. Cada suplemento incorpora la información
de su artículo correspondiente. Si el artículo se encuentra en el perfil de artículos solicitado, se sobrescribe con la información que
se encuentra en dicho perfil. Este comando está pensado para mostrar la información tal y como la está mostrando la pantalla de
ventas (Los datos extraídos són los datos c)

##### Ruta de llamada

/API/SupplementsProfiles/GetPOSProfile

##### Parámetros de entrada

**SupplementsProfileID:** Código de perfil de suplementos que queremos consultar.

**ArticlesProfileID:** Código de perfil de artículos de donde vamos a sacar la información de los suplementos.

##### JSON de solicitud

{
"SupplementsProfileID": 1,
"ArticlesProfileID": 1
}

##### Parámetros de salida

**SupplementsPrifile (POSSupplementsProfile):** Estructura de datos definida a continuación:

**SupplementsProfile** : Estructura de datos de un perfil de suplementos

```
Id (Decimal) : Id. del perfil de suplementos solicitado.
```
```
Name (String) : Nombre del perfil de suplementos solicitado
```
```
Supplements (List(Of SupplementDataType)) : Lista de suplementos con la siguiente estructura SupplementDataType
Estructura de un suplemento visual. Estará formado por los siguientes campos:
```
```
Is_SAC_Article(B oolean) : Campo lógico que en caso de ser True indicará que el artículo en cuestión es de talla y
color y en caso de ser false, indicará que se no se trata de un artículo de talla y color.
```
```
Apply_Dct_Of_Article_In_SAC_Article(B oolean) : Campo lógico que indicará si se aplicarán los descuentos a los
artículos de talla y color. Campo sólo válido si la aplicación corresponde a la de talla y color. Si no es una aplicación
de talla y color este valor será false.
```
```
SeasonID(String): Cadena de texto que mostrará la temporada asignada al artículo. Campo sólo válido si la
aplicación corresponde a la de talla y color. Si no es una aplicación de talla y color este valor vendrá vacío.
```
```
SalePrice1(Decimal): Valor decimal que indicará el precio 1 de rebaja general (no el de las líneas de precios de talla
y color). Campo sólo válido si la aplicación corresponde a la de talla y color. Si no es una aplicación de talla y color
este valor será 0.
```
```
SalePrice2(Decimal): Valor decimal que indicará el precio 2 de rebaja general (no el de las líneas de precios de talla
y color). Campo sólo válido si la aplicación corresponde a la de talla y color. Si no es una aplicación de talla y color
este valor será 0.
```

**SalePrice3(Decimal):** Valor decimal que indicará el precio 3 de rebaja general (no el de las líneas de precios de talla
y color). Campo sólo válido si la aplicación corresponde a la de talla y color. Si no es una aplicación de talla y color
este valor será 0.

**DctSalePrice1(Decimal):** Valor decimal que indicará el porcentaje 1 de rebaja general (no el de las líneas de precios
de talla y color). Campo sólo válido si la aplicación corresponde a la de talla y color. Si no es una aplicación de talla
y color este valor será 0.

**DctSalePrice2(Decimal):** Valor decimal que indicará el porcentaje 2 de rebaja general (no el de las líneas de precios
de talla y color). Campo sólo válido si la aplicación corresponde a la de talla y color. Si no es una aplicación de talla
y color este valor será 0.

**DctSalePrice3(Decimal):** Valor decimal que indicará el porcentaje 3 de rebaja general (no el de las líneas de precios
de talla y color). Campo sólo válido si la aplicación corresponde a la de talla y color. Si no es una aplicación de talla
y color este valor será 0.

**Apply_SAC_Prices(Boolean):** Campo lógico que en caso de ser True indicará que al artículo en cuestión se le deben
aplicar los precios definidos para las tallas y colores y no los precios genéricos. Campo sólo válido si la aplicación
corresponde a la de talla y color. Si no es una aplicación de talla y color este valor será false.

**OnSale(Boolean):** Campo lógico que indicará si el artículo está en rebajas. Campo sólo válido si la aplicación
corresponde a la de talla y color. Si no es una aplicación de talla y color este valor será 0.

**DeptCode** **_(integer)_** : Valor con el código de departamento de venta asociado al artículo.

**DeptDescription** **_(string)_** : Cadena con la descripción del departamento de venta asociado al artículo.

**MenuDish** **_(boolean)_** : Campo lógico que corresponde con el campo ‘Plato de Menú’ del artículo.

**WebArticle** **_(boolean)_** : Campo lógico que corresponde con el campo ‘Artículo Web’ del artículo.

**POS_SupplementsProfileID (decimal)** : Valor con el código de perfil de suplementos asociado al artículo.

**SelfOrdering_CommentsProfileID (decimal)** : Valor con el código de perfil de comentarios asociado a la
configuración de autocomanda del artículo.

**SelfOrdering_SupplementsProfileID (decimal)** : Valor con el código de perfil de suplementos asociado a la
configuración de autocomanda del artículo.

**POS_MenuID (decimal)** : Valor con el código de definición de menú asociado al artículo.

**POS_FastfoodID (decimal)** : Valor con el código de definición fastfood asociado al artículo.

**POS_PackID (decimal)** : Valor con el código de definición de pack asociado al artículo.

**Is_Inventoriable** **_(boolean)_** : Campo lógico que corresponde con el campo ‘Inventariable’ del artículo.

**TAVCode** **_(integer)_** : Valor con el código de IVA de venta asociado al artículo.

**TAVPer** **_(decimal)_** : Valor decimal con el porcentaje del IVA de venta asociado al artículo.

**AuxPrinters** **_(string)_** : Cadena con la secuencia de impresoras auxiliares asociadas al artículo.

**Commissionable** **_(boolean)_** : Campo lógico que corresponde con el campo ‘Artículo Comisionable’ del artículo.

**ModifiablePrice** **_(boolean)_** : Campo lógico que corresponde con el campo ‘Precio Modificable’ del artículo.


**DontPrintTicketValue0** **_(boolean)_** : Campo lógico que corresponde con el campo ‘No Imprimir en Factura si Valor 0’
del artículo.

**Weight** **_(boolean)_** : Campo lógico que corresponde con el campo ‘Artículo de Peso’ del artículo.

**DontNotifyUnitsPrice0** **_(boolean)_** : Campo lógico que corresponde con el campo ‘No Avisar si unidades o Precio es
0’ del artículo.

**NotifyModifyPriceUnits** **_(boolean)_** : Campo lógico que corresponde con el campo ‘Avisar para Modificar Precio y/o
Unidades’ del artículo.

**TwoForOne** **_(boolean)_** : Campo lógico que corresponde con el campo ‘Artículo Dos x Uno’ del artículo.

**POS_CommentsProfileID (decimal)** : Valor con el código de perfil de comentarios asociado al artículo.

**ErrorMessage** **_(string)_** : Cadena que almacenará posibles errores. Si la consulta se realizó correctamente, estará
vacía.

**PriceConfirmation** **_(boolean)_** : Campo lógico que corresponde con el campo ‘Confirmación Precio’ del artículo.

**FreeDescription** **_(boolean)_** : Campo lógico que corresponde con el campo ‘Descripción Libre’ del artículo.

**ArtDescription** **_(string)_** : Cadena de texto que corresponde a la descripción del artículo (máx. 40 caracteres).

**Price1** **_(decimal)_** : Valor decimal con el PVP1 del artículo.

**Price2** **_(decimal)_** : Valor decimal con el PVP2 del artículo.

**Price3** **_(decimal)_** : Valor decimal con el PVP3 del artículo.

**Price4** **_(decimal)_** : Valor decimal con el PVP4 del artículo.

**Price5** **_(decimal)_** : Valor decimal con el PVP5 del artículo.

**Dct1** **_(decimal)_** : Valor decimal con el porcentaje de descuento 1 del artículo.

**Dct2** **_(decimal)_** : Valor decimal con el porcentaje de descuento 2 del artículo.

**Dct3** **_(decimal)_** : Valor decimal con el porcentaje de descuento 3 del artículo.

**Dct4** **_(decimal)_** : Valor decimal con el porcentaje de descuento 4 del artículo.

**Dct5** **_(decimal)_** : Valor decimal con el porcentaje de descuento 5 del artículo.

**GraphDescrip1** **_(string)_** : Cadena de texto que corresponde a la Descripción Gráfica de la Línea 1 del artículo.

**GraphDescrip2** **_(string)_** : Cadena de texto que corresponde a la Descripción Gráfica de la Línea 2 del artículo.

**GraphDescrip3** **_(string)_** : Cadena de texto que corresponde a la Descripción Gráfica de la Línea 3 del artículo.

**ExtendedArtDescription** **_(string)_** : Cadena de texto que corresponde a la Descripción Extendida del artículo.

**ArtCode** **_(long)_** : Valor numérico de entre 1 y 13 dígitos correspondiente al código de del artículo.


##### JSON de respuesta

{
"ErrorMessage": "",
"SupplementsProfile": {
"Id": 1.0,
"Name": "Perfil Suplementos 1",
"Supplements": [
{
"Is_SAC_Article": false,
"Apply_Dct_Of_Article_In_SAC_Article": false,
"SeasonID": "",
"SalePrice1": 0.0,
"SalePrice2": 0.0,
"SalePrice3": 0.0,
"DctSalePrice1": 0.0,
"DctSalePrice2": 0.0,
"DctSalePrice3": 0.0,
"Apply_SAC_Prices": false,
"OnSale": false,
"DeptCode": 1,
"DeptDescription": "REFRESCOS",
"MenuDish": true,
"WebArticle": true,
"POS_SupplementsProfileID": 0.0,
"SelfOrdering_CommentsProfileID": 0.0,
"SelfOrdering_SupplementsProfileID": 0.0,
"POS_MenuID": 0.0,
"POS_FastfoodID": 0.0,
"POS_PackID": 0.0,
"Is_Inventoriable": true,
"TAVCode": 1,
"TAVPer": 10.0,
"AuxPrinters": "",
"Commissionable": false,
"ModifiablePrice": true,
"DontPrintTicketValue0": false,
"Weight": false,
"DontNotifyUnitsPrice0": false,
"NotifyModifyPriceUnits": false,
"TwoForOne": false,
"POS_CommentsProfileID": 0.0,
"ErrorMessage": "",
"PriceConfirmation": false,
"FreeDescription": false,
"ArtDescription": "COCA-COLA",
"Price1": 1.05,
"Price2": 1.05,
"Price3": 1.05,
"Price4": 1.05,
"Price5": 1.05,
"Dct1": 0.0,
"Dct2": 0.0,
"Dct3": 0.0,
"Dct4": 0.0,
"Dct5": 0.0,
"GraphDescrip1": "",
"GraphDescrip2": "",
"GraphDescrip3": "",
"ExtendedArtDescription": "",
"ArtCode": 1001
},...
{
"Is_SAC_Article": false,
"Apply_Dct_Of_Article_In_SAC_Article": false,
"SeasonID": "",


"SalePrice1": 0.0,
"SalePrice2": 0.0,
"SalePrice3": 0.0,
"DctSalePrice1": 0.0,
"DctSalePrice2": 0.0,
"DctSalePrice3": 0.0,
"Apply_SAC_Prices": false,
"OnSale": false,
"DeptCode": 1,
"DeptDescription": "REFRESCOS",
"MenuDish": true,
"WebArticle": false,
"POS_SupplementsProfileID": 0.0,
"SelfOrdering_CommentsProfileID": 0.0,
"SelfOrdering_SupplementsProfileID": 0.0,
"POS_MenuID": 0.0,
"POS_FastfoodID": 0.0,
"POS_PackID": 0.0,
"Is_Inventoriable": true,
"TAVCode": 1,
"TAVPer": 10.0,
"AuxPrinters": "",
"Commissionable": false,
"ModifiablePrice": true,
"DontPrintTicketValue0": false,
"Weight": false,
"DontNotifyUnitsPrice0": false,
"NotifyModifyPriceUnits": false,
"TwoForOne": false,
"POS_CommentsProfileID": 0.0,
"ErrorMessage": "",
"PriceConfirmation": false,
"FreeDescription": false,
"ArtDescription": "SCHWEPPES NARANJA",
"Price1": 1.05,
"Price2": 1.05,
"Price3": 1.05,
"Price4": 1.05,
"Price5": 1.05,
"Dct1": 0.0,
"Dct2": 0.0,
"Dct3": 0.0,
"Dct4": 0.0,
"Dct5": 0.0,
"GraphDescrip1": "",
"GraphDescrip2": "",
"GraphDescrip3": "",
"ExtendedArtDescription": "",
"ArtCode": 1003
}
]
}
}


### GetSupplementsProfiles

Mediante este comando se podrá obtener una estructura con todos los perfiles de suplementos con código entre Profile1 y Profile2,
cada uno con sus suplementos. Los suplementos son artículos del TPV, por eso la lista de suplementos es tan solo una lista de códigos
de artículos.

##### Ruta de llamada

/API/SupplementsProfiles/GetProfiles

##### Parámetros de entrada

**Profile1** **_(integer)_** : Número entero entre 1 y 999. Código inicial de perfil de suplementos a devolver. Si no se especifica, se devolverán
todos los perfiles desde el primero hasta el especificado en **Profile2**.

**Profile2** **_(integer)_** : Número entero entre 1 y 999. Código final de perfil de suplementos a devolver. Si no se especifica, se devolverán
todos los perfiles desde el especificado en **Profile1** hasta el último.

##### JSON de solicitud

{
"Profile1": 1,
"Profile2": 3
}

##### JSON de respuesta

Response
=======
{
"ErrorMessage": "",
"SupplementsProfiles": [
{
"Id": 1,
"Name": "INGREDIENTES SUPERHAMBURGUESA",
"Supplements": [
{
"ArtCode": 21001
},
{
"ArtCode": 21002
},
{
"ArtCode": 21003
},
{
"ArtCode": 21005
}
]
},
{
"Id": 2,
"Name": "INGREDIENTES HAMBURGUESA ANGUS",
"Supplements": [
{
"ArtCode": 21001
},
{


"ArtCode": 21002
},
{
"ArtCode": 21004
},
{
"ArtCode": 21006
}
]
},
{
"Id": 3,
"Name": "SALSAS",
"Supplements": [
{
"ArtCode": 21101
},
{
"ArtCode": 21102
},
{
"ArtCode": 21103
},
{
"ArtCode": 21104
}
]
},
{
"Id": 4,
"Name": "INGREDIENTES POLLO",
"Supplements": [
{
"ArtCode": 21201
},
{
"ArtCode": 21202
},
{
"ArtCode": 21203
},
{
"ArtCode": 21204
}
]
},
{
"Id": 5,
"Name": "INGREDIENTES PULPO",
"Supplements": [
{
"ArtCode": 21301
},
{
"ArtCode": 21302
},
{
"ArtCode": 21303
}
]
}
]
}


## Categoría Talla y Color

En esta categoría se incluirán todos los comandos que permitirán interactuar con datos de específicos de la aplicación talla y color.
Los comandos disponibles para esta categoría serán los siguientes:

1. **GetInfoSAC** : Obtiene la información de las dimensiones de talla y color existente en la base de datos.
2. **GetItemSAC** : Obtiene la información de talla y color para un artículo dado.


### GetInfoSAC

En función de un código alternativo de entrada, que puede ser ArtAltern (con valor) y ArtAlternBDP (con valor cero) o bien ArtAltern
(vacío) y ArtAlternBDP (con valor mayor que cero), mediante este comando se podrá buscar el código principal del artículo y da la
información de las tres dimensiones de talla y color. Si se facilitan simultáneamente ArtAltern y ArtAlternBDP, devuelve un mensaje
de error, igual que si ambos no tienen valor. También se devuelve un mensaje de error si no existe el código alternativo especificado
en la base de datos.

##### Ruta de llamada

/API/Sac/GetInfo

##### Parámetros de entrada

**ArtAltern(** **_string_** **)** : Cadena de texto con un máximo de 30 caracteres (puede estar vacío) que indica el código alternativo definido en
la línea de precios de talla y color del que queremos obtener información.

**ArtAlternBDP(** **_integer_** **)** : Valor entero mayor o igual que cero, que indica el código alternativo BDP autogenerado del que deseamos
obtener información.

##### JSON de solicitud

{
"ArtAltern": null,
"ArtAlternBDP": 1
}

##### Parámetros de salida

**ArtCode** (long): Valor numérico entero que indica el código principal del artículo.

**D1** (decimal): Valor numérico entero (puede valer cero) que indica la primera dimensión del artículo en la base de datos.

**D2** (decimal): Valor numérico entero (puede valer cero) que indica la segunda dimensión del artículo en la base de datos.

**D3** (decimal): Valor numérico entero (puede valer cero) que indica la tercera dimensión del artículo en la base de datos.

**NameD1** (string): Valor de texto de hasta 10 caracteres (puede estar vacío) que indica el nombre de la primera dimensión del artículo
en la base de datos (p.ej., la talla: 85).

**NameD2** (string): Valor de texto de hasta 10 caracteres (puede estar vacío) que indica el nombre de la segunda dimensión del artículo
en la base de datos (p.ej. el color: ROJO).

**NameD3** (string): Valor de texto de hasta 10 caracteres (puede estar vacío) que indica el nombre de la tercera dimensión del artículo
en la base de datos (p.ej. la copa: C).

**ErrorMessage** (string): Cadena de errores. Si todo ha ido bien, estará vacía. Contiene errores generales de validación.

#### JSON de respuesta:

{
"ArtCode": 1001,


"D1": 2.0,
"D2": 1.0,
"D3": 1.0,
"NameD1": "85",
"NameD2": "A",
"NameD3": "BLANCO",
"ErrorMessage": ""
}


### GetItemSAC

Mediante este comando se podrá obtener información exhaustiva sobre el desglose de talla y color de un artículo. El artículo debe
existir, ser de talla y color y tener al menos una dimensión activada.

##### Ruta de llamada

/API/Sac/GetItem

##### Parámetros de entrada

**ArtCode(** **_long_** **)** : Número entero de 1 a 13 dígitos mayor que cero. Corresponde al código del artículo del que se desea obtener
información.

##### JSON de solicitud

{
"ArtCode": 1001
}

##### Parámetros de salida

**Description(** **_string_** **)** : Campo de texto que contiene la descripción del artículo.

**InfoD1(** **_SACType_** **)** : Campo estructurado que contiene información de la primera dimensión del artículo.

**InfoD2(** **_SACType_** **)** : Campo estructurado que contiene información de la segunda dimensión del artículo.

**InfoD3(** **_SACType_** **)** : Campo estructurado que contiene información de la tercera dimensión del artículo.

**ErrorMessage(** **_string_** **)** : Cadena de errores. Si todo ha ido bien, estará vacía.

Los campos tipo **SACType** Tienen la siguiente estructura:

```
Active( booleano ) : Campo lógico que indica si la dimensión está activa o no.
```
```
DValue( decimal ) : Campo numérico que indica el valor de la dimensión (1, 2 ó 3).
```
```
DName( string ) : Campo de texto que indica el nombre de la dimensión (por ejemplo: COLOR).
```
```
DText( list of string ) : Array de campos de texto con los nombres de cada Talla, Color, Copa, etc. Por ejemplo: ROJO, VERDE,
NEGRO, AZUL. El array puede estar vacío.
```
##### JSON de respuesta

{
"Description": "SUJETADOR CLASSIC",
"InfoD1": {
"Active": true,
"DValue": 1.0,
"DName": "TALLA",
"DText": [
"85",
"90",


```
]
},...
```
```
"95",
"100"
```
"InfoD3": {
"Active": true,
"DValue": 3.0,
"DName": "COLOR",
"DText": [
"BLANCO",
"NEGRO"
]
},
"ErrorMessage": ""
}


## Categoría Salones

En esta categoría se incluirán todos los comandos que permitirán interactuar con datos de específicos de los salones. Los comandos
disponibles para esta categoría serán los siguientes:

1. **GetRoomTables** : Obtiene una lista con los números de las mesas configuradas el rango de mesas de un salón.
2. **GetRoomsTables** : Obtiene los salones con los números de mesas configuradas en sus rangos de mesas.


### GetRoomTables

Dado un número de salón, esta función devuelve una lista de números correspondiente a las mesas configuradas en el rango de
mesas del salón. Si el salón no existe, devuelve un mensaje de error. Si el salón no tiene un rango de mesas configurado, devolverá
una lista vacía (todas las mesas de la 1 a la 999 serían válidas para BDP en dicho salón).

##### Ruta de llamada

/API/Room/GetTables

##### Parámetros de entrada

**Id (** **_integer_** **)** : Código del salón del que queremos obtener las mesas. Si no se especifica ninguno, se devolverán TODOS los salones
configurados en BDP-Net con sus rangos de mesas (equivalente a llamar la función **_GetRoomsTables_** sin parámetro de entrada).

##### JSON de solicitud

{
"Id": 1
}

##### Parámetros de salida

**Tables** **_List (Of integer_** **)** : Lista con los números de mesa del salón. Si el salón no tuviera un rango de mesas configurado, esta lista
estará vacía (todas las mesas de la 1 a la 999 serían válidas para ese salón).

**ErrorMessage** (string): Cadena de errores. Si todo ha ido bien, estará vacía.

##### JSON de respuesta

{
"Tables": [1,3,5,6,10,11,12,13,14,15,16,17,18,19,20,24,26],
"ErrorMessage": ""
}


### GetRoomsTables

Dado un número de salón, esta función devuelve una lista de números correspondiente a las mesas configuradas en el rango de
mesas del salón. Si el salón no existe, devuelve un mensaje de error. Si el salón no tiene un rango de mesas configurado, devolverá
una lista vacía (todas las mesas de la 1 a la 999 serían válidas para BDP en dicho salón).

##### Ruta de llamada

/API/Rooms/GetTables

##### Parámetros de entrada

**Ids** **_List (of integer_** **)** : Lista con los códigos de salones a obtener. Si no se indica ninguno, se devolverán TODOS los salones configurados
en BDP-Net.

##### JSON de solicitud

{
"Ids": [1, 2]
}

{
}

##### Parámetros de salida

**Rooms** **_List (Of Room_** **)** : Lista con los salones solicitados. Es una estructura de tipo **Room**.

**ErrorMessage** (string): Cadena de errores. Si todo ha ido bien, estará vacía.

##### JSON de respuesta

{
"Rooms": [
{
"Id": 1,
"Name": "Comedor",
"Tables": [1,2,3,4,5,6,7]
},
{
"Id": 2,
"Name": "Terraza",
"Tables": [21,22,23,24,25,26,27,28,29,30,31,32,33,34,35]
}
],
"ErrorMessage": ""
}


### CallWaiter

Esta función se encarga de sacar un mensaje emergente en el TPV indicando que se reclama la atención de un camarero para una
mesa y salón determinados.

##### Ruta de llamada

/API/Waiters/Call

##### Parámetros de entrada

**Table** **_(integer_** **)** : Número de mesa de donde se reclama a un camarero

**Room** **_(integer_** **)** : Número de salón de donde se reclama a un camarero.

##### JSON de solicitud

{
"Table": 2,
"Room": 1
}

##### Parámetros de salida

**ErrorMessage** **_(string):_** Cadena de errores. Si todo ha ido bien, estará vacía.

##### JSON de respuesta

{
"ErrorMessage": ""
}


## Categoría Series TPV

En esta categoría se incluirán todos los comandos que permitirán interactuar con datos de específicos de las series del TPV. Los
comandos disponibles para esta categoría serán los siguientes:

### 3. GetPOSSeriesList : Obtiene la lista de series del TPV al que nos estamos conectando.


### GetPOSSeriesList

Esta función devuelva una lista de series creadas en el TPV, cada serie incluye:

##### Ruta de llamada

/API/POSSeries/GetList

##### Parámetros de entrada

**No tiene**

##### JSON de solicitud

{}

##### Parámetros de salida

**Series** **_List (Of POSSerie_** **)** : Lista con todas las Series TPV

**ErrorMessage** (string): Cadena de errores. Si todo ha ido bien, estará vacía.

##### JSON de respuesta

```
{
"ErrorMessage": "",
"Series": [
{
"Code": "00001AL",
"ShortDescription": "1AL",
"Description": "1T Albaranes",
"DocumentType": 2
},
{
"Code": "00001TB",
"ShortDescription": "1TB",
"Description": "1T Factura Simplificada",
"DocumentType": 0
},
{
"Code": "00001TM",
"ShortDescription": "1TM",
"Description": "1T Facturas Simplificadas Mesa",
"DocumentType": 0
}
]
}
```

## Modelos

A continuación, se muestra la definición de los modelos usados en las anteriores funciones:


### Order

Objeto que contiene los datos de la comanda.

**MarketplaceOrderId** **_(string)_** : Opcional. Identificador de la comanda en la plataforma enlazada con BDP a través de WebLink.

**MarketId** **_(integer)_** : Opcional. Código que identifica a la plataforma donde se ha originado la comanda.

**MarketName** **_(string)_** : Opcional. Nombre de la plataforma donde se ha originado la comanda.

**PreparationTime** **_(string)_** : Opcional. Cadena con la hora de preparación de la comanda en formato ISO8601 (ej.: “2019- 12 -
15T16:31:22Z”). En caso de tratarse de una comanda de servicio a domicilio, ésta será la hora a la que el repartidor tendrá que
recogerla para ir a hacer la entrega.

**OrderId** **_(integer)_** : Opcional. Identificador de la comanda en el sistema que está enlazado con BDP mediante WebLink. Si no es
especifica ninguno (o es cero), BDP asignará un número que será devuelto en el parámetro **OrderId** de salida.

**PosId** **_(integer)_** : Código del terminal de BDP-Net en el que se creará la comanda. Debe ser un terminal válido (tiene que estar dado
de alta) en la configuración de BDP-Net.

**Type** **_(integer)_** : Tipo de comanda. Puede tener los siguientes valores:

```
0 : Barra / Ticket aparcado - para recoger (take away)
1 : Mesa
2 : Servicio a domicilio (delivery)
```
**RoomNumber** **_(integer)_** : Código de salón si se trata de una comanda de mesa ( _Type_ 1).

**TableNumber** **_(integer)_** : Número de mesa si se trata de una comanda de mesa ( _Type_ 1).

**Customer** **_(OrderCustomer)_** : Datos del cliente de la comanda. Es una estructura del tipo _OrderCustomer_.

**Items** **_List (of OrderItem)_** : Conjunto de líneas de la comanda. Estructuras de tipo _OrderItem_.

**Surcharge** **_(decimal)_** : Porcentaje o importe del posible recargo aplicado a la comanda.

**SurchargePct** **_(boolean)_** : Indica si el valor de _Surcharge_ es un porcentaje o un importe.

**SurchargeAmount** **_(decimal)_** : Parámetro de sólo lectura con la función **GetOrder**. Importe del recargo.

**SurchargePercentage** **_(decimal)_** : Parámetro de sólo lectura con la función **GetOrder**. Porcentaje del recargo.

**Discount** **_(decimal)_** : Porcentaje o importe del posible descuento general aplicado a la comanda. No puede ser superior al 100% ni
inferior a 0.

**DiscountPct** **_(boolean)_** : Indica si se ha especificado el descuento general como un porcentaje o como un importe.

**DiscountAmount** **_(decimal)_** : Parámetro de sólo lectura con la función **GetOrder**. Importe del descuento.

**DiscountPercentage** **_(decimal)_** : Parámetro de sólo lectura con la función **GetOrder**. Porcentaje de descuento.

**Tip** **_(decimal)_** : Valor de la propina.

**Total** **_(decimal)_** : Importe total de la comanda (suma de los totales de las líneas menos el descuento general más la propina).

**VATIncluded** **_(boolean)_** : Indica si los importes de la comanda tienen el impuesto (IVA) incluido.


**ExecutionTime** **_(string)_** : Opcional. Cadena con la hora a la que tiene que entregarse la comanda en formato ISO8601 (ej.: “2019- 12 -
15T16:31:22Z”). En caso de tratarse de una comanda de servicio a domicilio, ésta será la hora a la que el repartidor estará en casa
del cliente (o bien la hora a la que el cliente la pasará a recoger por el establecimiento).

**Status** **_(integer)_** : Parámetro de sólo lectura con la función **GetOrder** (se ignora el valor que se le pueda especificar en **CreateOrder** ).
Indica el estado actual de la comanda y puede tener los siguientes valores:

```
0 : Esperando validación por parte del establecimiento
1 : Aceptada por el establecimiento
2 : Cancelada
3 : Facturada
```
**Comments** **_(string)_** : Campo para indicar las posibles observaciones de la comanda. Tiene un límite interno de 120 caracteres.

**OpeningEmployee (** **_Employee):_** Parámetro de sólo lectura con la función **GetOrder**. Estructura de tipo _Employee_ con los datos del
empleado que ha empezado la comanda.

**LastEmployee (** **_Employee):_** Parámetro de sólo lectura con la función **GetOrder**. Estructura de tipo _Employee_ con los datos del
empleado que ha añadido la última línea.

**CreationDate** **_(string)_** : Parámetro de sólo lectura con la función **GetOrder**. Cadena con la fecha y hora de apertura de la comanda
en formato ISO8601 (ej.: “2022- 03 - 07T09:48:03Z”).

**AlreadyInvoiced** **_(boolean)_** : Indica si el pedido se factura en la plataforma desde la cual se envía. Esto sirve para evitar una doble
imposición siempre que en BDP-Net se configure una serie de destino para este tipo de pedidos. Esto acarrea un conjunto de
restricciones sobre la comanda. Sirvan los siguientes casos de ejemplo:

```
· Tiene que estar pagada en su totalidad
· No se le podrán añadir más artículos ni eliminar ninguno
· No se podrán modificar líneas de la comanda (cambiar unidades, modificar el precio, invitar, etc.)
· No se podrán añadir ni modificar descuentos ni recargos.
```
**Payments** **_List (of OrderPayment)_** : Lista con los posibles pagos de la comanda. Si la comanda viene total o parcialmente pagada aquí
se indicará la forma de pago y el importe de los distintos pagos. Se podrán especificar un máximo de 3 pagos, igual que en el TPV
táctil de BDP-Net. Son elementos del tipo _OrderPayment_.

### OrderCustomer

Datos del cliente de la comanda.

**Id** **_(integer)_** : Código de cliente. Debe ser un código de cliente existente en la base de datos de BDP-Net.

**Name** **_(string)_** : Nombre del cliente.

**AddressStreet** **_(string)_** : Dirección del cliente (calle).

**AddressNumber** **_(string)_** : Dirección del cliente (número de puerta).

**ZipCode** **_(string)_** : Código postal del cliente.

**City** **_(string)_** : Población del cliente.

**Region** **_(string)_** : Provincia del cliente.

**Country** **_(string)_** : Nombre del país del cliente.


**LandlineNumber** **_(string)_** : Número de teléfono fijo.

**MobileNumber** **_(string)_** : Número de teléfono móvil.

**Email** **_(string)_** : Dirección de correo electrónico.

**TaxId** **_(string)_** : Número de identificación fiscal (NIF) del cliente.

**TaxType** **_(integer)_** : Campo de tipo numérico que permitirá especificar el tipo de documento de identificación del cliente. Los tipos
de documentos equivalentes a los valores son:

```
1 : N.I.F.
2 : N.I.F. Extranjero
3 : Pasaporte
4 : ID en País de Residencia
5 : Certificado Residencia
6 : Otro Documento
```
### OrderItem

Datos de una línea de la comanda.

**Lin** **_(integer?)_** : Número de línea de comanda (identificador único). Opcional. Se requiere para relacionar líneas con un _OrderItemType_
específico (menús, fastfoods y packs) con su desglose en _OrderItemTypeMetaInfo_.

**Id** **_(long)_** : Número natural de 13 dígitos. Corresponde al código principal del artículo en la base de datos de BDP-Net.

**Name** **_(string)_** : Descripción del artículo.

**Units** **_(decimal)_** : Unidades de la línea. No pueden ser cero ni iguales o superiores, en valor absoluto, a cien millones.

**Price** **_(decimal)_** : Precio unitario de la línea. No puede ser negativo.

**Supplement** **_(decimal)_** : Importe del total de los posibles suplementos de la línea.

**Discount** **_(decimal)_** : Porcentaje o importe del posible descuento aplicado a la línea. No puede ser superior al 100% ni inferior a 0.

**DiscountPct** **_(boolean)_** : Indica si el valor de _Discount_ es un porcentaje o un importe.

**DiscountAmount** **_(decimal)_** : Parámetro de sólo lectura con la función **GetOrder**. Importe del descuento.

**DiscountPercentage** **_(decimal)_** : Parámetro de sólo lectura con la función **GetOrder**. Porcentaje de descuento.

**Invitation** **_(boolean)_** : Devuelve o establece si la línea de comanda es una invitación.

**ProportionNumber** **_(integer)_** : Devuelve o establece el número de proporción del artículo. Si el artículo no tiene proporciones su
valor será 0. Si el artículo tiene proporciones su valor será el número de proporción (de la 1 a la 9).

**Total** **_(decimal)_** : Importe total de la línea (unidades x precio + suplemento - descuento).

**VatPct** **_(decimal)_** : Porcentaje de IVA de venta de la línea.

**TaxName** **_(string)_** : Parámetro de sólo lectura con la función **GetOrder**. Nombre del impuesto tal y como esté configurado en los
parámetros generales de BDP-Net: IVA, IGIC...

**Comments** **_List (of OrderItemComment)_** : Conjunto de comentarios de una línea de la comanda, del tipo _OrderItemComment_.


**Supplements** **_List (of OrderItemSupplement)_** : Conjunto de artículos de suplemento de una línea de la comanda, del tipo
_OrderItemSupplement_.

**OrderItemType** **_(integer)_** : Tipo de línea de comanda. Puede tener los siguientes valores:

```
0 : Standard (valor por defecto)
1 : Menu (en OrderItemTypeMetaInfo se especificará el desglose del menú)
2 : Fastfood (en OrderItemTypeMetaInfo se especificará el desglose del fastfood)
3 : Pack (en OrderItemTypeMetaInfo se especificará el desglose del pack)
4 : Talla y color (en TyC_D1 , TyC_D2 , TyC_D3 se especificará la talla y color del artículo)
```
**OrderItemTypeMetaInfo** **_(string)_** : Información serializada en JSON que corresponde al detalle de línea con una estructura
determinada en función del _OrderItemType_ establecido, cuando el valor de éste es 1, 2 ó 3 (menú, fastfood o pack).

**TyC_D1: (** **_integer_** **):** Parámetro únicamente válido presentará utilidad para la aplicación de talla y color, y cuando el _OrderItem_ tenga
desglose de tallas o colores ( _OrderItemType_ = 4). Valor para la dimensión 1 de la línea.

**TyC_D2: (** **_integer_** **)** Parámetro que únicamente presentará utilidad para la aplicación de talla y color, y cuando el _OrderItem_ tenga
desglose de tallas o colores ( _OrderItemType_ = 4). Valor para la dimensión 2 de la línea.

**TyC_D3: (** **_integer_** **):** Parámetro que únicamente presentará utilidad para la aplicación de talla y color, y cuando el _OrderItem_ tenga
desglose de tallas o colores ( _OrderItemType_ = 4). Valor para la dimensión 3 de la línea.

**OnSale (** **_boolean):_** Parámetro que únicamente presentará utilidad para la aplicación de talla y color.

**Employee (** **_Employee):_** Parámetro sólo de salida con la función **GetOrder**. Estructura de tipo _Employee_ con los datos del empleado
que ha creado la línea.

**Status** **_(integer)_** : Parámetro de sólo lectura con la función **GetOrder**. Indica el estado actual de la línea y puede tener los siguientes
valores:

```
0 : Esperando validación por parte del establecimiento
1 : Aceptada por el establecimiento
```
### OrderItemComment

Datos de una línea de comentarios de una línea de comanda.

**Id** **_(long)_** : Código de comentario definido en BDP-Net. Si no existe se creará como un comentario libre.

**Name** **_(string)_** : Descripción del comentario.

**Units** **_(decimal)_** : Unidades de la línea de comentario. Debe ser un número positivo.

### OrderItemSupplement

Datos de una línea de suplementos de una línea de comanda.

**Id** **_(long)_** : Número natural de 13 dígitos. Corresponde al código principal del artículo en la base de datos de BDP-Net.

**Name** **_(string)_** : Descripción del artículo.

**Units** **_(decimal)_** : Unidades de la línea de suplementos. Debe ser un número positivo.

**Price** **_(decimal)_** : Precio unitario de la línea. No puede ser negativo.


**Total** **_(decimal)_** : Importe total de la línea de suplementos (unidades x precio).

**Comments** **_List (of OrderItemComment)_** : Conjunto de comentarios de la línea de suplementos, del tipo _OrderItemComment_.

### OrderItemTypeMetaInfo

Cuando un _OrderItem_ es un menú, un fastfood o un pack, el campo _OrderItem.OrderItemType_ es igual a 1, 2 ó 3 respectivamente y
en _OrderItemTypeMetaInfo_ se indica el detalle (los componentes del menú, fastfood o pack) con una cadena que contiene,
serializada, esta información siguiendo la estructura correspondiente. La estructura es la siguiente:

**XOrder_MenuDataType** : Estructura con el detalle de los platos del menú seleccionados. Tiene los siguientes elementos:

```
Items List (of XOrder_MenuItemDataType) : Conjunto de platos/elementos elegidos en el menú. Son elementos del tipo
XOrder_MenuItemDataType , que tiene la siguiente estructura:
```
```
Group (integer) : Número del 1 al 6 que corresponde al grupo de platos al que pertenece el artículo (entrantes,
segundos platos, postres...).
```
```
GroupName (string) : Parámetro de sólo lectura con la función GetOrder. Nombre del grupo de platos del menú.
```
```
Id (long) : Código de artículo de un plato/elemento del menú.
```
```
Description (string) : Descripción del plato/elemento del menú.
```
```
Units (decimal) : unidades seleccionadas del elemento del menú.
```
```
Supplement (decimal) : suplemento (precio unitario) del plato de menú.
```
```
Comments List (of OrderItemComment) : Conjunto de comentarios del plato del menú. Estructuras del tipo
OrdrItemCommens.
```
**XOrder_FastfoodItemDataType** : Estructura con el detalle de los ingredientes del fastfood seleccionados. Tiene los siguientes
elementos:

```
FixedItems List (of XOrder_FastfoodItemDataType) : Lista con el conjunto de ingredientes fijos del fastfood. Son elementos
que vienen incluídos en el fastfood (por ejemplo, el tomate de una pizza) aunque el cliente pueda no querer alguno (por
ejemplo “sin queso”). Son elementos del tipo XOrder_FastfoodItemDataType , que tiene la siguiente estructura:
```
```
Id (long) : código de artículo de un ingrediente del fastfood.
```
```
Description (string) : descripción del ingrediente del fastfood.
```
```
Units (decimal) : unidades seleccionadas del ingrediente del fastfood.
```
```
VariableItems List (of XOrder_FastfoodItemDataType) : lista con el conjunto de ingredientes variables del fastfood. Son los
elementos elegidos por el cliente de entre todos los ingredientes que puede tener el fastfood. Es posible indicar más de
una unidad de un mismo ingrediente. Son elementos del tipo XOrder_FastfoodItemDataType , igual que los FixedItems.
```
**XOrder_PackDataType** : Estructura con el detalle de los elementos del pack seleccionados. Tiene los siguientes elementos:

```
Items List (of XOrder_PackItemDataType) : Lista con el conjunto de elementos elegidos en el pack. Son elementos del tipo
XOrder_PackItemDataType , que tiene la siguiente estructura:
```
```
Group (integer) : Número del 1 al 6 que corresponde al grupo del pack al que pertenece el artículo (ensalada,
hamburguesa, patatas fritas, bebida...).
```

```
GroupName (string) : Parámetro de sólo lectura con GetOrder. Nombre del grupo del pack.
```
```
Id (long) : Código de artículo de un elemento del pack.
```
```
Description (string) : Descripción del elemento del pack.
```
```
Units (decimal) : Unidades seleccionadas del elemento del pack.
```
```
Price (decimal) : Precio unitario del elemento del pack.
```
### Employee

Datos de un empleado.

**Id** **_(long)_** : Código de empleado definido en BDP-Net.

**Name** **_(string)_** : Nombre del empleado.

**Salesperson** **_(boolean)_** : Indica si se trata de un empleado vendedor/camarero (los empleados de las comandas siempre lo son) o no
(cocinero, administración...).

### OrderPayment

Datos de un pago de la comanda.

**TenderId** **_(integer)_** : Número natural que corresponde al código de la forma de pago definida en BDP-Net.

**TenderName** **_(string)_** : Parámetro de sólo lectura con la función **GetOrder**. Descripción de la forma de pago.

**Amount** **_(decimal)_** : Importe del pago.

**PaymentId** **_(string)_** : Código identificador del pago en el sistema donde se ha hecho la comanda.

### InvoiceParameters

Datos para emitir la factura de la comanda.

**InvoiceEmailAddress** **_(string)_** : Dirección de correo electrónico a la que se puede enviar la factura como fichero PDF usando el diseño
de factura del terminal de BDP-Net en el que se emita la factura. Por defecto se enviaría a la dirección de correo del cliente que
pueda ya tener la comanda.

**PrintTicket** **_(boolean)_** : Indica si quiere que se imprima la factura emitida por la impresora de facturas que tenga configurada el
terminal de facturación y con el diseño de la serie en la que se emite la factura. Por defecto, su valor es _false_.

**BillingDetails** **_(InvoiceBillingDetails)_** : Objeto del tipo _InvoiceBillingDetails_ con los datos de facturación del cliente. Por defecto se
usarán los datos del cliente que pueda ya tener la comanda.

### InvoiceBillingDetails

Datos para emitir la factura de la comanda.

**Name** **_(string)_** : Nombre del cliente.

**Address** **_(string)_** : Dirección del cliente.


**ipCode** **_(string)_** : Código postal del cliente.

**City** **_(string)_** : Población del cliente.

**TaxId** **_(string)_** : Número de identificación fiscal (NIF) del cliente.

**TaxType** **_(integer)_** : Campo de tipo numérico que permitirá especificar el tipo de documento de identificación del cliente. Los tipos
de documentos equivalentes a los valores son:

```
1 : N.I.F.
2 : N.I.F. Extranjero
3 : Pasaporte
4 : ID en País de Residencia
5 : Certificado Residencia
6 : Otro Documento
```
### OrderIdentifier

Parámetros que identifican a la comanda.

**OrderId** **_(integer)_** : Número que identifica la comanda.

**MarketId** **_(integer)_** : Número que identifica a la plataforma donde se ha originado la comanda. Coincide con el parámetro
especificado en la llamada a _CreateOrder_.

**MarketplaceOrderId** **_(string)_** : Identificador de la comanda en la plataforma.

**RoomNumber** **_(integer)_** : Número que identifica el salón de la comanda cuando se trata de una mesa.

**TableNumber** **_(integer)_** : Número que identifica el número de mesa de la comanda.

### POS

Datos de un terminal.

**Id** **_(integer)_** : Código del terminal.

**Name** **_(string)_** : Descripción del terminal.

### Room

Datos de un salón.

**Id** **_(integer)_** : Código de salón.

**Name** **_(string)_** : Nombre del salón.

**Tables** **_List (Of integer_** **)** : Lista con los números de mesa del salón. Si el salón no tuviera un rango de mesas configurado, esta lista
estaría vacía (todas las mesas de la 1 a la 999 serían válidas para ese salón).

### Tender

Datos de una forma de pago.


**Id** **_(integer)_** : Código de la forma de pago en BDP-Net.

**Name** **_(string)_** : Descripción de la forma de pago.

**Type** **_(integer)_** : Tipo de forma de pago. Se corresponde al campo Tipo de las formas de pago de BDP-Net.

### POSSerie

Datos de una serie de TPV.

**Código** : Código de la serie

**Descripción abreviada** : Campo de texto limitado a 10 caracteres que permitirá especificar un descripción corta de la serie.

**Descripción** : Descripción de la serie

**Tipo de documento** : Tipo de documento al cual va referida la serie, éste dato puede ir del 0 al 6 (ambos incluidos)

```
0 = Factura simplificadas
1 = Factura rectificativa
2 = Albaranes
3 = Facturas de albarán
4 = Albaranes Traspaso Hotel
5 = Albaranes Tickets Cero
6 = Facturas Substitutivas
```

