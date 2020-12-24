<?php

require_once('EspoApiClient.php');
require_once('credentials.php');

//Initialize the API client
global $client;
$client = new EspoApiClient($url);
$client->setApiKey($apiKey);

//Get the query parameters from the post request
$selectorContact = explode(",", $_POST['contactQuery']);
$selectorAccountProduct = explode(",", $_POST['accountProductQuery']);
$selectorAccountType = explode(",", $_POST['accountTypeQuery']);
$accountLocationType = explode(",", $_POST['accountLocationTypeQuery']);
$provinceVisitAddress = explode(',', $_POST['provinceVisitAddress']);
$cityVisitAddress = explode(',', $_POST['cityVisitAddress']);

//An empty array to store the results in
$result = [];

//Get all accounts that match or query parameters
$accounts = getAccounts($selectorAccountProduct, 
                        $selectorAccountType,
                        $accountLocationType,
                        $provinceVisitAddress);

//Get the values, stored in the 'list' object
$accountValues = $accounts['list'];

$knownCitiesFile = fopen('known_cities.json', 'r');
$knownCities = json_decode(fread($knownCitiesFile, filesize('known_cities.json')), true);

//Loop over all the accounts,
//Get the associated contacts
foreach($accountValues as $account) {

    //Check if the account is in any of the cities the user specified (if they specified any)
    if(!in_array('Ignore Filter', $cityVisitAddress) && !empty($cityVisitAddress)) {
        $satisfied = false;
        
        //Convert the account's city name to lower case
        $accountCityLowercase = strtolower($account['shippingAddressCity']);
        
        //Loop over all the cities provided by the user
        foreach($cityVisitAddress as $city) {

            //Turn the user provided city to lower case
            $lowerCity = strtolower($city);

            //Since we're looping over cities anyway, add them to the list of cities we
            //already know, so the user doesn't have to type it in manuall next time
            //$cityVisitAddress is located in apiValues.php
            if(!in_array($lowerCity, $knownCities['known_cities'])) {
                array_push($knownCities['known_cities'], $lowerCity);
            }

            //Check if the user provided city and the account's city are the same
            //If yes, set the satisfied boolean to true, and break out of this loop
            if($lowerCity === $accountCityLowercase) {
                $satisfied = true;
                break;
            }
        }

        //Check if the city condition is me
        //If not, skip this Account
        if(!$satisfied) continue;
    }

    //Get the contacts for this account id
    //with the selected query parameters
    $contacts = getContacts($account['id'], $selectorContact, $client, in_array('Ignore Filter', $selectorContact));
    
    //Add the response to the results array
    array_push($result, [
        'accountId' => $account['id'],
        'accountProducten' => $account['producten'],
        'contacts' => $contacts
    ]);
}

//Encode the knownCities array back into JSON and store it back into the file
$knownCitiesJson = json_encode($knownCities);

$knownCitiesFile = fopen('known_cities.json', 'w+');
fwrite($knownCitiesFile, $knownCitiesJson);
fclose($knownCitiesFile);

//Return the results array as JSON
echo json_encode($result);

function getAccounts($product, $type, $locationType, $province) {
    
    //Default query parameters, used for all queries
    $where = [
        [
            'type' => 'isFalse',
            'attribute' => 'exrelatie'
        ]
    ];

    //If we do not ignore the product query,
    //add it to the 'where'
    if(!in_array('Ignore Filter', $product)) {
        array_push($where, [
            'type' => 'arrayAnyOf',
            'attribute' => 'producten',
            'value' => $product
        ]);
    }

    //If we do not ignore the account type query,
    //Add it to the 'where'
    if(!in_array('Ignore Filter', $type)) {
        array_push($where, [
            'type' => 'arrayAnyOf',
            'attribute' => 'relatieType',
            'value' => $type
        ]);
    }

    //If we do not ignore the account's location type query,
    //add it to 'where'
    if(!in_array('Ignore Filter', $locationType)) {
        array_push($where, [
            'type' => 'arrayAnyOf',
            'attribute' => 'relatieType',
            'value' => $locationType
        ]);
    }

    //If we do not ignore the account's visit address' privince query,
    //add it to the 'where'
    if(!in_array('Ignore Filter', $province)) {
        array_push($where, [
            'type' => 'in',
            'attribute' => 'shippingAddressState',
            'value' => $province
        ]);
    }

    //Set up the parameters
    $params = [
        'offset' => 0,
        'where' => $where,
        'orderBy' => 'createdAt',
        'order' => 'desc',
        'select' => 'id,producten,shippingAddressCity'
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

//Function used to get a contact from the API based on the Account ID
function getContacts($accountId, $selectorContact, $client, $ignoreSelector) {
    
    //Default query parameters, used for all queries
    $where = [
        [
            'type' => 'linkedWith',
            'attribute' => 'account',
            'value' => $accountId
        ],
        [
            'type' => 'isFalse',
            'attribute' => 'emailAddressIsOptedOut'
        ]
    ];
    
    //If we do not ignore the query parameter,
    //add it to the 'where'
    if(!$ignoreSelector) {
        array_push($where, [
            'type' => 'arrayAnyOf',
            'attribute' => 'role',
            'value' => $selectorContact
        ]);
    }
    
    //Set up the parameters
    $params = [
        'offset' => 0,
        'where' => $where,
        'orderBy' => 'createdAt',
        'order' => 'desc'
    ];
    
    //Make the request
    $response = '';
    try {
        $response = $GLOBALS['client']->request('GET', 'Contact', $params);
    
    } catch(Exception $e) {
        $response = $e->getCode();
    }

    return $response;
}