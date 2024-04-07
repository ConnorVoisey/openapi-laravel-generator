<?php

/**
 *  THIS FILE WAS AUTO GENERATED,
 *  DO NOT MODIFY IT,
 *  ANY CHANGES SHOULD BE MADE IN THE OPENAPI SPEC
 *  THEN RE-GENERATE THIS FILE
 */
namespace App\Http\Requests;

use Illuminate\Foundation\Http\FormRequest;

class {{class_name}} extends FormRequest
{
    public static function getRules({% for field in fields %}
        array ${{field.name}} = [],{% endfor %}
    ): array {
        return [{% for field in fields %}
            '{{field.name}}' => [
            {% for rule in field.rules %}   {{rule}}, 
            {% endfor %}    ...${{field.name}}
            ],
            {% endfor %}
        ];
    }
}
