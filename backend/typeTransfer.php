<?php
require_once('EspoApiClient.php');
require_once('credentials.php');

//Initialize the API client
global $client;
$client = new EspoApiClient($url);
$client->setApiKey($apiKey);

$accounts = getAccounts();

$transfers = [
    'Customer' => 'Klant',
    'Prospect' => 'Prospect',
    'Investor' => 'Investeerder',
    'Partner' => 'Partner',
    'Reseller' => 'Reseller',
    'Supplier' => 'Leverancier',
    'Government' => '(Semi) Overheid',
    'utility company' => 'Klant',
    'EX-Relatie' => 'EX-Relatie'
];

foreach($accounts['list'] as $account) {
    $id = $account['id'];
    $type = $account['type'];
    
    echo $type . ";";
    if($type === "Klant") {
        continue;
    } 

    $typeTransfered = $transfers[$type];

    echo($typeTransfered);
}

function put($id, $payload) {
    $response = '';
    try {
        $response = $GLOBALS['client']->request('PUT', 'Account', $payload);
    
    } catch(Exception $e) {
        $response = $e->getCode();
    }
}

function getAccounts() {
    $where = [
        [
            'type' => 'isNotNull',
            'attribute' => 'type'
        ],
        [
            'type' => 'notEquals',
            'attribute' => 'type',
            'value' => ''
        ]
    ];
    
    //Set up the parameters
    $params = [
        'offset' => 0,
        'where' => $where,
        'orderBy' => 'id',
        'order' => 'desc',
        'select' => 'id,type'
    ];

    //Make the request
    $response = '';
    try {
        $response = $GLOBALS['client']->request('GET', 'Account', $params);
    
    } catch(Exception $e) {
        $response = $e->getCode();
    }

    return $response;
}


?>